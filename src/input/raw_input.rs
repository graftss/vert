use bevy::prelude::NonSendMut;

use super::input::{HidAxisId, HidButtonId, HidHatSwitchId, HidId, InputValue};

// A bevy system to poll the `RawInputManager` regularly so that it stays up-to-date
// with all rawinput joysticks.
#[cfg(target_os = "windows")]
pub fn poll_rawinput_system(mut raw_input: NonSendMut<RawInputRes>) {
    raw_input.0.poll_events();
}

pub trait RawInputReader {
    fn poll_events(&mut self) -> usize;
    fn num_joysticks(&self) -> usize;
    fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue>;
    fn poll_hid_axis(&mut self, id: &HidId, axis_id: &HidAxisId) -> Option<InputValue>;
    fn poll_hid_hatswitch(&mut self, id: &HidId, hat_id: &HidHatSwitchId) -> Option<InputValue>;
}

// The non-send bevy resource for HID/DirectInput gamepads.
#[cfg(target_os = "windows")]
pub struct RawInputRes(RawInput);

#[cfg(target_os = "windows")]
impl Default for RawInputRes {
    fn default() -> Self {
        RawInputRes()
    }
}

#[cfg(target_os = "windows")]
pub struct RawInput {
    manager: multiinput::RawInputManager,
}

#[cfg(target_os = "windows")]
impl Default for RawInput {
    fn default() -> Self {
        use multiinput::*;

        println!("initialize raw input manager");

        // Initialize the `multiinput::RawInputManager` to only listen to non-xinput joysticks.
        // These should be the only input devices not covered by the default bevy input plugin.
        let mut manager = RawInputManager::new().unwrap();
        manager.register_devices(DeviceType::Joysticks(XInputInclude::False));

        RawInput { manager }
    }
}

#[cfg(target_os = "windows")]
impl RawInputReader for RawInput {
    // Syncs the state of the `RawInputManager` based on pending rawinput events.
    fn poll_events(&mut self) -> usize {
        // Calling `get_events` updates the joystick state of `RawInputManager`.
        self.manager.get_events().into_iter().count()
    }

    // Returns the number of joysticks in the device list.
    fn num_joysticks(&self) -> usize {
        self.manager.get_device_list().joysticks.len()
    }

    fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue> {
        let js = self.manager.get_joystick_state(*id)?;

        Some(InputValue::Button(js.button_states[*button_id]))
    }

    // Reads the value of a `HidAxis` input source from the rawinput state.
    fn poll_hid_axis(&mut self, id: &HidId, axis_id: &multiinput::Axis) -> Option<InputValue> {
        use multiinput::Axis;

        let js = self.manager.get_joystick_state(*id)?;

        let axis_state = match *axis_id {
            Axis::X => js.axis_states.x,
            Axis::Y => js.axis_states.y,
            Axis::Z => js.axis_states.z,
            Axis::RX => js.axis_states.rx,
            Axis::RY => js.axis_states.ry,
            Axis::RZ => js.axis_states.rz,
            Axis::SLIDER => js.axis_states.slider,
        };

        axis_state.map(InputValue::Axis)
    }

    // Reads the value of a `HidHatSwitch` input source from the raw input state.
    fn poll_hid_hatswitch(
        &mut self,
        id: &HidId,
        hat_id: &self::HidHatSwitchId,
    ) -> Option<InputValue> {
        use self::HidHatSwitchId::*;
        use multiinput::HatSwitch;

        let js = self.manager.get_joystick_state(*id)?;
        let active_hatswitch = js.hatswitch?;

        let is_hat_id_down = match active_hatswitch {
            HatSwitch::Center => *hat_id == Center,
            HatSwitch::Right => *hat_id == Right,
            HatSwitch::Left => *hat_id == Left,
            HatSwitch::Up => *hat_id == Up,
            HatSwitch::Down => *hat_id == Down,
            HatSwitch::UpRight => *hat_id == Up || *hat_id == Right,
            HatSwitch::DownRight => *hat_id == Down || *hat_id == Right,
            HatSwitch::DownLeft => *hat_id == Down || *hat_id == Left,
            HatSwitch::UpLeft => *hat_id == Up || *hat_id == Left,
        };

        Some(InputValue::Button(is_hat_id_down))
    }
}

#[cfg(target_os = "macos")]
pub struct RawInputRes(pub NoopRawInput);

impl Default for RawInputRes {
    fn default() -> Self {
        RawInputRes(NoopRawInput)
    }
}

#[cfg(target_os = "macos")]
pub struct NoopRawInput;

#[cfg(target_os = "macos")]
impl RawInputReader for NoopRawInput {
    fn poll_events(&mut self) -> usize {
        0
    }
    fn num_joysticks(&self) -> usize {
        0
    }
    fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue> {
        None
    }
    fn poll_hid_axis(&mut self, id: &HidId, axis_id: &HidAxisId) -> Option<InputValue> {
        None
    }
    fn poll_hid_hatswitch(&mut self, id: &HidId, hat_id: &HidHatSwitchId) -> Option<InputValue> {
        None
    }
}
