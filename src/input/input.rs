use bevy::prelude::*;

use super::raw_input::{RawInputReader, RawInputRes};

#[derive(Debug)]
pub enum InputValue {
    Axis(f64),
    Button(bool),
}

pub type HidId = usize;
pub type HidButtonId = usize;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HidHatSwitchId {
    Center,
    Up,
    Right,
    Down,
    Left,
}

// Duplicate of multiinput::Axis
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HidAxisId {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    SLIDER,
}

pub enum InputSource {
    Key(KeyCode),
    Button(GamepadButton),
    Axis(GamepadAxis),
    HidButton(HidId, HidButtonId),
    HidAxis(HidId, HidAxisId),
    HidHatSwitch(HidId, HidHatSwitchId),
}

// Poll each `InputSource` in `sources`, storing the results as a vector of `Option<InputValue>`.
pub fn poll_input_sources(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_inputs: NonSendMut<RawInputRes>,
    sources: Vec<InputSource>,
) -> Vec<Option<InputValue>> {
    use self::InputSource::*;

    let mut result = Vec::new();

    for source in sources.iter() {
        match source {
            &Key(key_code) => {
                let pressed = keyboard_input.pressed(key_code);
                result.push(Some(InputValue::Button(pressed)));
            }
            &Button(button) => {
                let pressed = button_input.pressed(button);
                result.push(Some(InputValue::Button(pressed)));
            }
            &Axis(axis) => {
                if let Some(value) = axis_input.get(axis) {
                    result.push(Some(InputValue::Axis(value as f64)));
                } else {
                    result.push(None);
                }
            }
            HidAxis(id, axis) => result.push(raw_inputs.0.poll_hid_axis(&id, &axis)),
            HidButton(id, button) => result.push(raw_inputs.0.poll_hid_button(&id, &button)),
            HidHatSwitch(id, hatswitch) => {
                result.push(raw_inputs.0.poll_hid_hatswitch(&id, &hatswitch))
            }
        }
    }

    result
}

pub fn test_gamepad_system(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_inputs: NonSendMut<RawInputRes>,
) {
    use InputSource::*;

    let sources = vec![
        Key(KeyCode::W),
        HidAxis(0, HidAxisId::X),               // left stick x axis
        HidHatSwitch(0, HidHatSwitchId::Right), // dpad right
        HidButton(0, 2),                        // x button
    ];

    let values = poll_input_sources(
        keyboard_input,
        button_input,
        axis_input,
        raw_inputs,
        sources,
    );

    println!("values: {:?}", values);
}
