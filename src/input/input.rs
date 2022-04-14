use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::raw_input_reader::*;
use super::RawInputReader;

#[derive(Debug, Clone, Copy)]
pub enum InputValue {
    Axis(f32),
    Button(bool),
}

pub type HidId = usize;
pub type HidButtonId = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HidHatSwitchId {
    Center,
    Up,
    Right,
    Down,
    Left,
}

// Duplicate of multiinput::Axis
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HidAxisId {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
    SLIDER,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AxisSign {
    Plus,
    Minus,
}

impl AxisSign {
    pub fn clamp_f32(&self, axis_value: f32) -> f32 {
        match self {
            AxisSign::Plus => axis_value.clamp(0.0, 1.0),
            AxisSign::Minus => -axis_value.clamp(0.0, -1.0),
        }
    }

    pub fn clamp_f64(&self, axis_value: f64) -> f32 {
        match self {
            AxisSign::Plus => axis_value.clamp(0.0, 1.0) as f32,
            AxisSign::Minus => -axis_value.clamp(-1.0, 0.0) as f32,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputSource {
    Key(KeyCode),
    Button(GamepadButton),
    Axis(GamepadAxis, AxisSign),
    HidButton(HidId, HidButtonId),
    HidAxis(HidId, HidAxisId, AxisSign),
    HidHatSwitch(HidId, HidHatSwitchId),
}

// Poll each `InputSource` in `sources`, storing the results as a vector of `Option<InputValue>`.
pub fn poll_input_sources(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_input: NonSendMut<RawInputRes>,
    sources: Vec<&InputSource>,
) -> Vec<Option<InputValue>> {
    use self::InputSource::*;

    let mut result = Vec::new();

    for source in sources.iter() {
        match source {
            &&Key(key_code) => {
                let pressed = keyboard_input.pressed(key_code);
                result.push(Some(InputValue::Button(pressed)));
            }
            &&Button(button) => {
                let pressed = button_input.pressed(button);
                result.push(Some(InputValue::Button(pressed)));
            }
            &&Axis(axis, sign) => {
                if let Some(value) = axis_input.get(axis) {
                    let clamped_value = sign.clamp_f32(value);
                    result.push(Some(InputValue::Axis(clamped_value)));
                } else {
                    result.push(None);
                }
            }
            HidAxis(id, axis, sign) => result.push(raw_input.0.poll_hid_axis(&id, &axis, &sign)),
            HidButton(id, button) => result.push(raw_input.0.poll_hid_button(&id, &button)),
            HidHatSwitch(id, hatswitch) => {
                result.push(raw_input.0.poll_hid_hatswitch(&id, &hatswitch))
            }
        }
    }

    result
}

#[derive(Component)]
pub struct InputSink {
    pub sources: Vec<InputSource>,
    pub values: Vec<Option<InputValue>>,
}

impl InputSink {
    pub fn new(sources: Vec<InputSource>) -> InputSink {
        let size = sources.len();
        InputSink {
            sources,
            values: vec![None; size],
        }
    }
}

// Mutate each `InputSink` component with the current value of the input source
// given by its `InputSource` field.
pub fn resolve_input_sinks_system(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    raw_input: NonSendMut<RawInputRes>,
    mut query: Query<&mut InputSink>,
) {
    let mut sources = Vec::new();

    // Collect the input sources from all sinks.
    for sink in query.iter() {
        for source in sink.sources.iter() {
            sources.push(source);
        }
    }

    // Poll the value of each input source
    let input_values =
        poll_input_sources(keyboard_input, button_input, axis_input, raw_input, sources);

    // Write those values to their associated sources
    let mut sink_start = 0;
    for mut sink in query.iter_mut() {
        let sink_len = sink.sources.len();
        for i in 0..sink_len {
            sink.values[i] = input_values[sink_start + i];
        }
        sink_start += sink_len;
    }
}

pub fn test_gamepad_system(
    keyboard_input: Res<Input<KeyCode>>,
    button_input: Res<Input<GamepadButton>>,
    axis_input: Res<Axis<GamepadAxis>>,
    mut raw_input: NonSendMut<RawInputRes>,
) {
    use InputSource::*;

    let sources = vec![
        &Key(KeyCode::W),
        &HidAxis(0, HidAxisId::X, AxisSign::Plus), // left stick x axis
        &HidHatSwitch(0, HidHatSwitchId::Right),   // dpad right
        &HidButton(0, 2),                          // x button
    ];

    let values = poll_input_sources(keyboard_input, button_input, axis_input, raw_input, sources);
}
