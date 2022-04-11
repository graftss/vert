use bevy::prelude::KeyCode;

#[derive(Debug)]
enum InputKind {
    Axis,
    Button,
}

#[derive(Debug)]
enum Input {
    Axis(f32),
    Button(bool),
}

impl Input {
    fn get_kind(&self) -> InputKind {
        match *self {
            Self::Axis(_) => InputKind::Axis,
            Self::Button(_) => InputKind::Button,
        }
    }
}

struct InputSourceKey {
    name: &'static str,
    input_kind: InputKind,
}

impl InputSourceKey {
    fn new_button(name: &'static str) -> InputSourceKey {
        InputSourceKey {
            name,
            input_kind: InputKind::Button,
        }
    }
}

struct InputSource {
    id: i64,
    name: String,
    keys: Vec<InputSourceKey>,
}

const NUM_KEYBOARD_KEYS: usize = 4;

#[derive(Debug)]
pub struct InputState {
    pub keyboard: [bool; NUM_KEYBOARD_KEYS],
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            keyboard: [false, false, false, false],
        }
    }
}

pub const KEYBOARD_KEYS: [KeyCode; 4] = [KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D];
