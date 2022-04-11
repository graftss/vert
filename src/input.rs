
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

struct ControllerKey {
  name: &'static str,
  input_kind: InputKind,
}

impl ControllerKey {
  fn new_button(name: &'static str) -> ControllerKey {
    ControllerKey { name, input_kind: InputKind::Button }
  }
}

struct Controller {
  id: i64,
  name: String,
  keys: Vec<ControllerKey>,
}

fn make_keyboard() -> Controller {
  Controller {
    id: 0,
    name: "keyboard".to_string(),
    keys: vec![
      ControllerKey::new_button("w"),
      ControllerKey::new_button("a"),
      ControllerKey::new_button("s"),
      ControllerKey::new_button("d"),
    ]
  }
}
