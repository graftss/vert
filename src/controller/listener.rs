use bevy::prelude::*;

use crate::input::{
    input::{AxisSign, InputSink, InputSource, GAMEPAD_AXES, MIN_LISTENABLE_AXIS_MAG},
    raw_input_reader::RawInputRes,
    RawInputReader,
};

use super::layout::{ControllerKey, ControllerLayout, ControllerLayoutsRes, PS2_KEY_ORDER};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ListenerState {
    Inactive,
    Listening,
    DoneListening,
}

pub struct InputListener {
    state: ListenerState,
    pub key: Option<ControllerKey>,
}

impl Default for InputListener {
    fn default() -> Self {
        InputListener {
            state: ListenerState::Inactive,
            key: None,
        }
    }
}

impl InputListener {
    pub fn start_listen(&mut self, key: ControllerKey) {
        self.state = ListenerState::Listening;
        self.key = Some(key);
    }

    pub fn end_listen(&mut self) {
        self.state = ListenerState::DoneListening;
    }

    pub fn is_listening(&self) -> bool {
        self.state == ListenerState::Listening
    }
}

pub fn input_listener_system(
    mut input_listener: ResMut<InputListener>,
    mut layout: ResMut<ControllerLayoutsRes>,
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_input: NonSendMut<RawInputRes>,
    gamepads: Res<Gamepads>,
) {
    if input_listener.state != ListenerState::Listening {
        return;
    }

    let key = input_listener
        .key
        .unwrap_or_else(|| panic!("wacky `InputListener` state encountered"));

    // Listen for rawinput
    if let Some(rawinput_source) = raw_input.0.listen() {
        layout.set_binding(key, &rawinput_source);
        input_listener.end_listen();
        return;
    }

    // Listen for keyboard input
    for keycode in keyboard_input.get_just_pressed() {
        layout.set_binding(key, &InputSource::Key(*keycode));
        input_listener.end_listen();
        return;
    }

    // Listen for xinput buttons
    for b in button_input.get_just_pressed() {
        layout.set_binding(key, &InputSource::Button(*b));
        input_listener.end_listen();
        return;
    }

    // Listen for xinput axes
    for gamepad in gamepads.iter() {
        for axis in GAMEPAD_AXES {
            let gamepad_axis = GamepadAxis(*gamepad, axis);
            let axis_input = axis_input.get(gamepad_axis);
            let axis_source = match axis_input {
                Some(f) if f > (MIN_LISTENABLE_AXIS_MAG as f32) => {
                    Some(InputSource::Axis(gamepad_axis, AxisSign::Plus))
                }
                Some(f) if f < (-MIN_LISTENABLE_AXIS_MAG as f32) => {
                    Some(InputSource::Axis(gamepad_axis, AxisSign::Minus))
                }
                _ => None,
            };

            if let Some(source) = axis_source {
                layout.set_binding(key, &source);
                input_listener.end_listen();
                return;
            }
        }
    }
}

pub fn cleanup_input_listener_system(
    mut input_listener: ResMut<InputListener>,
    mut sinks: Query<&mut InputSink>,
) {
    if input_listener.state != ListenerState::DoneListening {
        for mut sink in sinks.iter_mut() {
            sink.sources_dirty = true;
        }
    }
}
