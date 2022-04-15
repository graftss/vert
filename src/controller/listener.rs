use bevy::prelude::*;

use crate::input::{
    input::{AxisSign, InputSource, GAMEPAD_AXES, MIN_LISTENABLE_AXIS_MAG},
    raw_input_reader::RawInputRes,
    RawInputReader,
};

use super::layout::{ControllerLayout, ControllerLayoutRes, PS2_KEY_ORDER};

pub struct InputListener {
    pub listening: bool,
    pub key_idx: u8,
}

impl Default for InputListener {
    fn default() -> Self {
        InputListener {
            listening: false,
            key_idx: 0,
        }
    }
}

impl InputListener {
    pub fn start_listen(&mut self, key_idx: u8) {
        self.listening = true;
        self.key_idx = key_idx;
    }

    pub fn end_listen(&mut self) {
        self.listening = false;
    }
}

pub fn input_listener_system(
    mut listener_state: ResMut<InputListener>,
    mut layout: ResMut<ControllerLayoutRes>,
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_input: NonSendMut<RawInputRes>,
    gamepads: Res<Gamepads>,
) {
    if !listener_state.listening {
        return;
    }

    let key = PS2_KEY_ORDER[listener_state.key_idx as usize];

    // Listen for rawinput
    if let Some(rawinput_source) = raw_input.0.listen() {
        layout.set_binding(key, &rawinput_source);
        listener_state.end_listen();
        return;
    }

    // Listen for keyboard input
    for keycode in keyboard_input.get_just_pressed() {
        layout.set_binding(key, &InputSource::Key(*keycode));
        listener_state.end_listen();
        return;
    }

    // Listen for xinput buttons
    for b in button_input.get_just_pressed() {
        layout.set_binding(key, &InputSource::Button(*b));
        listener_state.end_listen();
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
                listener_state.end_listen();
                return;
            }
        }
    }
}
