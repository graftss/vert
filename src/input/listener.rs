use std::fs::OpenOptions;

use bevy::prelude::*;

use crate::{
    controller::layout::{ControllerKey, ControllerLayout, ControllerLayoutsRes},
    editor::inspector::InputSinkId,
    input::{
        input::{AxisSign, InputSink, InputSource, GAMEPAD_AXES, MIN_LISTENABLE_AXIS_MAG},
        raw_input_reader::RawInputRes,
        RawInputReader,
    },
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ListenerState {
    // Not listening for input.
    Inactive,
    // Actively listening for the next input.
    ListenInputSource,
    // Actively listening for the next *already bound* controller key input.
    ListenControllerKey,
}

// The consumers who can invoke the listener to capture the next heard input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListenerConsumer {
    Key(ControllerKey),
    Sink(InputSinkId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenerResult {
    SourceToKey(InputSource, ControllerKey),
    KeyToSink(ControllerKey, InputSinkId),
}

pub struct InputListener {
    state: ListenerState,
    consumer: Option<ListenerConsumer>,
    pub result: Option<ListenerResult>,
}

impl Default for InputListener {
    fn default() -> Self {
        InputListener {
            state: ListenerState::Inactive,
            consumer: None,
            result: None,
        }
    }
}

impl InputListener {
    // Start listening for a new binding.
    // This is used to register an input source to a controller key.
    pub fn listen_input_source(&mut self, key: ControllerKey) {
        self.state = ListenerState::ListenInputSource;
        self.consumer = Some(ListenerConsumer::Key(key));
    }

    pub fn listening_for_input_source(&self) -> bool {
        self.state == ListenerState::ListenInputSource
    }

    pub fn has_key_consumer(&self, key: ControllerKey) -> bool {
        self.consumer == Some(ListenerConsumer::Key(key))
    }

    // Start listening for input from an already-bound `ControllerKey`.
    // This is used to register a controller key to an input display.
    pub fn listen_for_controller_key(&mut self, sink: InputSinkId) {
        self.state = ListenerState::ListenControllerKey;
        self.consumer = Some(ListenerConsumer::Sink(sink));
    }

    pub fn listening_for_controller_key(&self) -> bool {
        self.state == ListenerState::ListenControllerKey
    }

    pub fn has_sink_consumer(&self, sink_id: InputSinkId) -> bool {
        self.consumer == Some(ListenerConsumer::Sink(sink_id))
    }

    // Using both the `InputListener` and an `EventReader` from the
    // inspector UI is awkward, so instead the UI will read the
    // listener result from here.
    pub fn consume_result(&mut self) -> Option<ListenerResult> {
        let result = self.result;
        self.result = None;
        result
    }

    pub fn stop_listening(&mut self) {
        self.state = ListenerState::Inactive;
        self.consumer = None;
    }
}

pub fn listen_for_input_source(
    keyboard: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut raw: NonSendMut<RawInputRes>,
    gamepads: Res<Gamepads>,
) -> Option<InputSource> {
    // Listen for rawinput
    if let Some(rawinput_source) = raw.0.listen() {
        return Some(rawinput_source);
    }

    // Listen for keyboard input
    for keycode in keyboard.get_just_pressed() {
        return Some(InputSource::Key(*keycode));
    }

    // Listen for xinput buttons
    for b in buttons.get_just_pressed() {
        return Some(InputSource::Button(*b));
    }

    // Listen for xinput axes
    for gamepad in gamepads.iter() {
        for axis in GAMEPAD_AXES {
            let gamepad_axis = GamepadAxis(*gamepad, axis);
            let axis_input = axes.get(gamepad_axis);

            match axis_input {
                Some(f) if f > (MIN_LISTENABLE_AXIS_MAG as f32) => {
                    return Some(InputSource::Axis(gamepad_axis, AxisSign::Plus))
                }
                Some(f) if f < (-MIN_LISTENABLE_AXIS_MAG as f32) => {
                    return Some(InputSource::Axis(gamepad_axis, AxisSign::Minus))
                }
                _ => (),
            };
        }
    }

    None
}

pub fn input_listener_system(
    mut input_listener: ResMut<InputListener>,
    layouts: ResMut<ControllerLayoutsRes>,
    keyboard: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    raw: NonSendMut<RawInputRes>,
    gamepads: Res<Gamepads>,
    mut event_writer: EventWriter<ListenerResult>,
) {
    match input_listener.state {
        ListenerState::ListenInputSource => {
            // Stop listening on any heard input source.
            if let Some(source) = listen_for_input_source(keyboard, buttons, axes, raw, gamepads) {
                if let Some(ListenerConsumer::Key(key)) = input_listener.consumer {
                    // TODO: change this to an event
                    event_writer.send(ListenerResult::SourceToKey(source, key));
                } else {
                    panic!("weird input listener state");
                }
            }
        }
        ListenerState::ListenControllerKey => {
            // Stop listening when a bound controller key is heard.
            if let Some(source) = listen_for_input_source(keyboard, buttons, axes, raw, gamepads) {
                if let Some(key) = layouts.is_source_bound(&source) {
                    if let Some(ListenerConsumer::Sink(sink)) = input_listener.consumer {
                        let result = ListenerResult::KeyToSink(key, sink);
                        event_writer.send(result);
                        input_listener.result = Some(result);
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn cleanup_input_listener_system(
    input_listener: ResMut<InputListener>,
    mut sinks: Query<&mut InputSink>,
) {
    if input_listener.state != ListenerState::Inactive {
        for mut sink in sinks.iter_mut() {
            sink.sources_dirty = true;
        }
    }
}
