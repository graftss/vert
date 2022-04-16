use bevy::prelude::*;

use crate::VERSION;

// All possible app states.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AppState {
    // Edit an input display
    Editor,
    // Configure input bindings to controllers
    ConfigureController,
    // Present an input display
    Present,
}

const INITIAL_STATE: AppState = AppState::Editor;

struct AppStateParams {
    pub hotkey: KeyCode,
    pub state: AppState,
    pub title: &'static str,
}

pub struct RequestStateEvent(AppState);

const STATE_PARAMS: [AppStateParams; 3] = [
    AppStateParams {
        hotkey: KeyCode::F2,
        state: AppState::Editor,
        title: "editor",
    },
    AppStateParams {
        hotkey: KeyCode::F3,
        state: AppState::ConfigureController,
        title: "controller setup",
    },
    AppStateParams {
        hotkey: KeyCode::F4,
        state: AppState::Present,
        title: "presentation",
    },
];

fn find_params(state: AppState) -> &'static AppStateParams {
    match STATE_PARAMS.iter().find(|p| p.state == state) {
        Some(p) => p,
        None => panic!("undefined params for state"),
    }
}

// Contains behavior that should occur every time a state is entered.
fn generic_state_transition(mut windows: ResMut<Windows>, state: AppState) {
    println!("success changing appstate: {:?}", state);
    let params = find_params(state);

    // Attempt to change the window title
    if let Some(window) = windows.get_primary_mut() {
        window.set_title(format!("vert {} [{}]", VERSION, params.title));
    }
}

pub fn state_transition_system(
    mut app_state: ResMut<State<AppState>>,
    mut er_reqstate: EventReader<RequestStateEvent>,
    mut windows: ResMut<Windows>,
) {
    for RequestStateEvent(state) in er_reqstate.iter() {
        if *app_state.current() == *state {
            continue;
        }

        // Attempt to change state.
        if let Err(state_err) = app_state.set(*state) {
            // Failed to change state.
            panic!("failure changing appstate: {:?}", state_err);
        }

        // Succeeded changing states.
        generic_state_transition(windows, *state);

        // Ignore more than the first successful state change per system execution.
        break;
    }
}

pub fn state_hotkey_system(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut ew_reqstate: EventWriter<RequestStateEvent>,
) {
    let current_state = *app_state.current();

    for AppStateParams {
        hotkey: key, state, ..
    } in STATE_PARAMS
    {
        if (keyboard_input.pressed(key) && state != current_state) {
            ew_reqstate.send(RequestStateEvent(state));
        }
    }
}

pub fn initial_state_transition(mut windows: ResMut<Windows>) {
    generic_state_transition(windows, INITIAL_STATE);
}

pub fn add_state_systems(app: &mut App) {
    app.add_event::<RequestStateEvent>();
    app.add_state(INITIAL_STATE);
    app.add_system(state_hotkey_system);
    app.add_system(state_transition_system);
    app.add_startup_system(initial_state_transition);
}
