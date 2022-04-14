use bevy::prelude::*;

use crate::{input::input::RESOLVE_INPUT_LABEL, AppState};

use super::{
    analog_stick::{
        add_analog_stick_teardown_system, analog_stick_display_system, spawn_analog_stick,
    },
    button::{add_button_teardown_system, button_update_system, spawn_button},
    display::{AtomicDisplay, Display},
};

pub fn display_startup_system(mut commands: Commands, display_data: Res<Display>) {
    // Spawn each `AtomicDisplay` in the `Display` resource.
    for atom in display_data.atoms.iter() {
        match atom {
            AtomicDisplay::Button(data) => spawn_button(&mut commands, data),
            AtomicDisplay::AnalogStick(data) => spawn_analog_stick(&mut commands, data),
        }
    }
}

pub fn add_display_systems(app: &mut App, display_state: AppState) {
    // Startup
    app.add_system_set(SystemSet::on_enter(display_state).with_system(display_startup_system));

    // Update
    app.add_system_set(
        SystemSet::on_update(display_state)
            .with_system(button_update_system.after(RESOLVE_INPUT_LABEL))
            .with_system(analog_stick_display_system.after(RESOLVE_INPUT_LABEL)),
    );

    // Teardown
    add_button_teardown_system(app, display_state);
    add_analog_stick_teardown_system(app, display_state);
}
