use bevy::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AppState {
    Display,
    ConfigureController,
}

struct AppStateHotkey {
    pub key: KeyCode,
    pub state: AppState,
}

const STATE_HOTKEYS: [AppStateHotkey; 2] = [
    AppStateHotkey {
        key: KeyCode::F2,
        state: AppState::Display,
    },
    AppStateHotkey {
        key: KeyCode::F3,
        state: AppState::ConfigureController,
    },
];

pub fn state_hotkey_system(
    mut app_state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let current_state = *app_state.current();

    for AppStateHotkey { key, state } in STATE_HOTKEYS {
        if (keyboard_input.pressed(key) && state != current_state) {
            if let Err(state_err) = app_state.set(state) {
                println!("state hotkey error: {:?}", state_err);
            } else {
                println!("changed appstate: {:?}", state);
            }
        }
    }
}
