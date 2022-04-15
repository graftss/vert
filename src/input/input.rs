use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::controller::layout::ControllerKey;
use crate::controller::layout::ControllerLayoutsRes;
use crate::controller::listener::InputListener;

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
    sources: Vec<&Option<InputSource>>,
) -> Vec<Option<InputValue>> {
    use self::InputSource::*;

    sources
        .iter()
        .map(|wrapped_source| match wrapped_source {
            None => None,
            Some(Key(key_code)) => {
                let pressed = keyboard_input.pressed(*key_code);
                Some(InputValue::Button(pressed))
            }
            Some(Button(button)) => {
                let pressed = button_input.pressed(*button);
                Some(InputValue::Button(pressed))
            }
            Some(Axis(axis, sign)) => axis_input
                .get(*axis)
                .map(|value| InputValue::Axis(sign.clamp_f32(value))),
            Some(HidAxis(id, axis, sign)) => raw_input.0.poll_hid_axis(&id, &axis, &sign),
            Some(HidButton(id, button)) => raw_input.0.poll_hid_button(&id, &button),
            Some(HidHatSwitch(id, hatswitch)) => raw_input.0.poll_hid_hatswitch(&id, &hatswitch),
        })
        .collect()
}

#[derive(Component)]
pub struct InputSink {
    // The abstract controller keys associated with this sink.
    // Updating `keys` will automatically propagate to both `sources` and then `values`.
    pub keys: Vec<ControllerKey>,

    // The concrete input sources associated with this sink.
    // Updating `sources` will automatically propagate to `values`.
    pub sources: Vec<Option<InputSource>>,

    // The concrete input values associated with the sources in `sources`.
    // Automatically updated by the input sink resolution system.
    pub values: Vec<Option<InputValue>>,

    // A flag indicating that `sources` is not synced with `keys`.
    // The input sink resolution system will resync these vectors during the next
    // execution of the input sink resolution system.
    pub sources_dirty: bool,
}

impl InputSink {
    pub fn new(keys: Vec<ControllerKey>) -> InputSink {
        let size = keys.len();
        InputSink {
            keys,
            sources: vec![None; size],
            values: vec![None; size],
            sources_dirty: true,
        }
    }
}

// Mutate each `InputSink` with the `sources_dirty` flag set to `true`.
// Update a dirty sink's `sources` vectors by mapping each entry of its `keys` vector
// to the binding found in the global controller layout resource.
pub fn resolve_dirty_sources_system(
    layouts: Res<ControllerLayoutsRes>,
    mut query: Query<&mut InputSink>,
) {
    for mut sink in query.iter_mut() {
        if !sink.sources_dirty {
            continue;
        }

        // Collect the `InputSource` bindings associated to each `ControllerKey`.
        let mut bindings = vec![];
        for &key in sink.keys.iter() {
            bindings.push(layouts.get_binding(key));
        }

        // Write those bindings to the `InputSink`.
        for (i, key) in bindings.iter().enumerate() {
            sink.sources[i] = *key;
        }

        sink.sources_dirty = false;
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
        let sink_len = sink.keys.len();
        for i in 0..sink_len {
            sink.values[i] = input_values[sink_start + i];
        }
        sink_start += sink_len;
    }
}

const POLL_INPUT_TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum InputSystemLabel {
    PollRawinput,
    ResolveDirtySources,
    ResolveInputValues,
}

pub fn add_input_systems(app: &mut App) {
    // Add the rawinput polling system when targeting Windows.
    #[cfg(target_family = "windows")]
    {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(POLL_INPUT_TIME_STEP as f64))
                .with_system(
                    super::raw_input_reader::poll_rawinput_system
                        .label(InputSystemLabel::PollRawinput),
                ),
        );
    }

    // Add the global rawinput resource, which is a no-op on platforms besides Windows.
    app.init_non_send_resource::<RawInputRes>();

    // Add the input resolution system to write up-to-date input to `InputSink` components.
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(POLL_INPUT_TIME_STEP as f64))
            .with_system(resolve_dirty_sources_system.label(InputSystemLabel::ResolveDirtySources))
            .with_system(
                resolve_input_sinks_system
                    .label(InputSystemLabel::ResolveInputValues)
                    .after(InputSystemLabel::PollRawinput)
                    .after(InputSystemLabel::ResolveDirtySources),
            ),
    );
}
