use bevy::prelude::*;

#[derive(Debug)]
pub enum InputValue {
    Axis(f64),
    Button(bool),
}

// The non-send bevy resource for HID/DirectInput gamepads.
pub struct RawInputRes(pub RawInput);
pub struct RawInput {
    pub manager: multiinput::RawInputManager,
}

impl Default for RawInputRes {
    fn default() -> Self {
        use multiinput::*;

        println!("initialize raw input manager");

        // Initialize the `multiinput::RawInputManager` to only listen to non-xinput joysticks.
        // These should be the only input devices not covered by the default bevy input plugin.
        let mut manager = RawInputManager::new().unwrap();
        manager.register_devices(DeviceType::Joysticks(XInputInclude::False));

        RawInputRes(self::RawInput { manager })
    }
}

impl RawInput {
    // Syncs the state of the `RawInputManager` based on pending rawinput events.
    pub fn poll_events(&mut self) -> usize {
        // Calling `get_events` updates the joystick state of `RawInputManager`.
        self.manager.get_events().into_iter().count()
    }

    // Returns the number of joysticks in the device list.
    pub fn num_joysticks(&self) -> usize {
        self.manager.get_device_list().joysticks.len()
    }

    pub fn poll_hid_button(&mut self, id: &usize, button_id: &HidButtonId) -> Option<InputValue> {
        let js = self.manager.get_joystick_state(*id)?;

        Some(InputValue::Button(js.button_states[*button_id]))
    }

    // Reads the value of a `HidAxis` input source from the rawinput state.
    pub fn poll_hid_axis(&mut self, id: &usize, axis_id: &multiinput::Axis) -> Option<InputValue> {
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
    pub fn poll_hid_hatswitch(
        &mut self,
        id: &usize,
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

// A bevy system to poll the `RawInputManager` regularly so that it stays up-to-date
// with all rawinput joysticks.
pub fn poll_rawinput_system(mut raw_inputs: NonSendMut<RawInputRes>) {
    raw_inputs.0.poll_events();
}

pub type HidId = usize;
pub type HidButtonId = usize;
pub type HidAxisId = multiinput::Axis;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HidHatSwitchId {
    Center,
    Up,
    Right,
    Down,
    Left,
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
        HidAxis(0, multiinput::Axis::X), // left stick x axis
        HidHatSwitch(0, HidHatSwitchId::Right), // dpad right
        HidButton(0, 2),                 // x button
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
