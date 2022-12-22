use super::input::{
    AxisSign, HidAxisId, HidButtonId, HidHatSwitchId, HidId, InputSource, InputValue,
};

pub trait RawInputReader {
    fn update(&mut self, id: HidId);

    // Returns the `InputSource` of the first joystick event captured by the update, if such an event exists.
    fn listen(&mut self) -> Option<InputSource>;

    fn num_joysticks(&self) -> usize;
    fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue>;
    fn poll_hid_axis(
        &mut self,
        id: &HidId,
        axis_id: &HidAxisId,
        sign: &AxisSign,
    ) -> Option<InputValue>;
    fn poll_hid_hatswitch(&mut self, id: &HidId, hat_id: &HidHatSwitchId) -> Option<InputValue>;
}

#[cfg(target_os = "windows")]
pub mod windows {
    use std::time::Instant;

    use super::RawInputReader;
    use crate::{controller::layout::ControllerLayoutsRes, input::input::*};
    use bevy::prelude::{NonSendMut, Res};
    use multiinput::JoystickState;

    // A bevy system to poll the `RawInputManager`.
    // Polling is what sync the internal state of `RawInputManager` to the current rawinput.
    pub fn poll_rawinput_system(
        mut raw_input: NonSendMut<RawInputRes>,
        controller: Res<ControllerLayoutsRes>,
    ) {
        for (_, source) in controller.ps2.sources.iter() {
            if let Some(id) = source.get_hid_id() {
                raw_input.0.update(id);
                break;
            }
        }
    }

    // The non-send bevy resource for HID/DirectInput gamepads.
    pub struct RawInputRes(pub RawInput);

    impl Default for RawInputRes {
        fn default() -> Self {
            RawInputRes(RawInput::default())
        }
    }

    pub struct RawInput {
        manager: multiinput::RawInputManager,
        joystick_state: Option<JoystickState>,
    }

    impl Default for RawInput {
        fn default() -> Self {
            use multiinput::*;

            // Initialize the `multiinput::RawInputManager` to only listen to non-xinput joysticks.
            // These should be the only input devices not covered by the default bevy input plugin.
            let mut manager = RawInputManager::new().unwrap();
            manager.register_devices(DeviceType::Joysticks(XInputInclude::False));

            RawInput {
                manager,
                joystick_state: None,
            }
        }
    }

    impl RawInput {
        fn get_cached_joystick_state(&self, _id: HidId) -> Option<&JoystickState> {
            self.joystick_state.as_ref()
        }
    }

    impl super::RawInputReader for RawInput {
        // Syncs the state of the `RawInputManager` based on pending rawinput events.
        fn update(&mut self, id: HidId) {
            loop {
                // Reading the events one by one like this has the effect of also updating
                // the internal joystick state of `RawInputManager`.
                if let Some(_) = self.manager.get_event() {
                } else {
                    break;
                }
            }

            self.joystick_state = self.manager.get_joystick_state(id);
        }

        fn listen(&mut self) -> Option<InputSource> {
            use multiinput::RawEvent::*;
            use multiinput::State;
            loop {
                if let Some(e) = self.manager.get_event() {
                    match e {
                        JoystickButtonEvent(id, button_id, State::Pressed) => {
                            return Some(InputSource::HidButton(id, button_id));
                        }
                        JoystickAxisEvent(id, axis_id, value) => {
                            if value > MIN_LISTENABLE_AXIS_MAG {
                                return Some(InputSource::HidAxis(
                                    id,
                                    HidAxisId::from_multiinput_axis(axis_id),
                                    AxisSign::Plus,
                                ));
                            } else if value < -MIN_LISTENABLE_AXIS_MAG {
                                return Some(InputSource::HidAxis(
                                    id,
                                    HidAxisId::from_multiinput_axis(axis_id),
                                    AxisSign::Minus,
                                ));
                            } else {
                                continue;
                            }
                        }
                        JoystickHatSwitchEvent(id, mi_hatswitch) => {
                            if let Some(hatswitch) =
                                HidHatSwitchId::from_multiinput_hatswitch(mi_hatswitch)
                            {
                                return Some(InputSource::HidHatSwitch(id, hatswitch));
                            } else {
                                continue;
                            }
                        }
                        _ => continue,
                    }
                } else {
                    break;
                }
            }

            None
        }

        // Returns the number of joysticks in the device list.
        fn num_joysticks(&self) -> usize {
            self.manager.get_device_list().joysticks.len()
        }

        fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue> {
            let js = self.get_cached_joystick_state(*id)?;

            Some(InputValue::Button(js.button_states[*button_id]))
        }

        // Reads the value of a `HidAxis` input source from the rawinput state.
        fn poll_hid_axis(
            &mut self,
            id: &HidId,
            axis_id: &HidAxisId,
            axis_sign: &AxisSign,
        ) -> Option<InputValue> {
            let js = self.get_cached_joystick_state(*id)?;

            let axis_state = match *axis_id {
                HidAxisId::X => js.axis_states.x,
                HidAxisId::Y => js.axis_states.y,
                HidAxisId::Z => js.axis_states.z,
                HidAxisId::RX => js.axis_states.rx,
                HidAxisId::RY => js.axis_states.ry,
                HidAxisId::RZ => js.axis_states.rz,
                HidAxisId::SLIDER => js.axis_states.slider,
            };

            axis_state.map(|s| InputValue::Axis(axis_sign.clamp_f64(s)))
        }

        // Reads the value of a `HidHatSwitch` input source from the raw input state.
        fn poll_hid_hatswitch(
            &mut self,
            id: &HidId,
            hat_id: &self::HidHatSwitchId,
        ) -> Option<InputValue> {
            use self::HidHatSwitchId::*;
            use multiinput::HatSwitch;

            let js = self.get_cached_joystick_state(*id)?;

            let is_hat_id_down = match js.hatswitch.as_ref()? {
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
}

#[cfg(target_os = "macos")]
pub mod macos {
    use super::RawInputReader;
    use crate::input::input::*;
    pub struct RawInputRes(pub NoopRawInput);

    impl Default for RawInputRes {
        fn default() -> Self {
            RawInputRes(NoopRawInput)
        }
    }

    pub struct NoopRawInput;

    impl RawInputReader for NoopRawInput {
        fn update(&mut self) {}
        fn listen(&mut self) -> Option<InputSource> {
            None
        }
        fn num_joysticks(&self) -> usize {
            0
        }
        fn poll_hid_button(&mut self, id: &HidId, button_id: &HidButtonId) -> Option<InputValue> {
            None
        }
        fn poll_hid_axis(
            &mut self,
            id: &HidId,
            axis_id: &HidAxisId,
            sign: &AxisSign,
        ) -> Option<InputValue> {
            None
        }
        fn poll_hid_hatswitch(
            &mut self,
            id: &HidId,
            hat_id: &HidHatSwitchId,
        ) -> Option<InputValue> {
            None
        }
    }
}
