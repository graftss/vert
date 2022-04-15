use bevy::core::FixedTimestep;
use bevy::prelude::*;

use super::raw_input_reader::*;
use super::RawInputReader;

// The smallest axis magnitude that isn't ignored when listening for axis input.
pub const MIN_LISTENABLE_AXIS_MAG: f64 = 0.4;

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

impl HidHatSwitchId {
    // Map d-pad hatswitch rawinputs to corresponding `HidHatSwitchId` values.
    #[cfg(target_family = "windows")]
    pub fn from_multiinput_hatswitch(hatswitch: multiinput::HatSwitch) -> Option<HidHatSwitchId> {
        match hatswitch {
            multiinput::HatSwitch::Up => Some(HidHatSwitchId::Up),
            multiinput::HatSwitch::Right => Some(HidHatSwitchId::Right),
            multiinput::HatSwitch::Down => Some(HidHatSwitchId::Down),
            multiinput::HatSwitch::Left => Some(HidHatSwitchId::Left),
            _ => None,
        }
    }
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

impl HidAxisId {
    #[cfg(target_family = "windows")]
    pub fn from_multiinput_axis(axis: multiinput::Axis) -> HidAxisId {
        match axis {
            multiinput::Axis::X => HidAxisId::X,
            multiinput::Axis::Y => HidAxisId::Y,
            multiinput::Axis::Z => HidAxisId::Z,
            multiinput::Axis::RX => HidAxisId::RX,
            multiinput::Axis::RY => HidAxisId::RY,
            multiinput::Axis::RZ => HidAxisId::RZ,
            multiinput::Axis::SLIDER => HidAxisId::SLIDER,
        }
    }
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

impl InputSource {
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}

pub const GAMEPAD_AXES: [GamepadAxisType; 8] = [
    GamepadAxisType::LeftStickX,
    GamepadAxisType::LeftStickY,
    GamepadAxisType::LeftZ,
    GamepadAxisType::RightStickX,
    GamepadAxisType::RightStickY,
    GamepadAxisType::RightZ,
    GamepadAxisType::DPadX,
    GamepadAxisType::DPadY,
];

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

const POLL_INPUT_TIME_STEP: f32 = 1.0 / 60.0;

pub const POLL_RAWINPUT_LABEL: &'static str = "poll_rawinput";
pub const RESOLVE_INPUT_LABEL: &'static str = "resolve_input";

pub fn add_input_systems(app: &mut App) {
    // Add the rawinput polling system when targeting Windows.
    #[cfg(target_family = "windows")]
    {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(POLL_INPUT_TIME_STEP as f64))
                .with_system(
                    super::raw_input_reader::poll_rawinput_system.label(POLL_RAWINPUT_LABEL),
                ),
        );
    }

    // Add the global rawinput resource, which is a no-op on platforms besides Windows.
    app.init_non_send_resource::<RawInputRes>();

    // Add the input resolution system to write up-to-date input to `InputSink` components.
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(POLL_INPUT_TIME_STEP as f64))
            .with_system(
                resolve_input_sinks_system
                    .label(RESOLVE_INPUT_LABEL)
                    .after(POLL_RAWINPUT_LABEL),
            ),
    );
}
