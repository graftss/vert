use bevy::prelude::*;

use crate::{
    controller::layout::ControllerLayoutsRes,
    util::{read_from_file, write_to_file},
    AppState,
};

use super::{
    analog_stick::AnalogStickAtomicDisplay,
    button::ButtonAtomicDisplay,
    display::{AtomicInputDisplay, InputDisplayRes, TaggedAtomicParams},
    frame::FrameAtomicDisplay,
};

// Call the update and teardown systems for a list of atomic types.
macro_rules! add_atomic_display_systems {
    ($app:ident, $state:ident, $atomic_type:ty) => {
        <$atomic_type>::add_update_systems($app, $state);
        <$atomic_type>::add_teardown_systems($app, $state);
    };

    ($app:ident, $state:ident, $atomic_type:ty $(, $rest:ty)+) => {{
        add_atomic_display_systems!($app, $state, $atomic_type);
        add_atomic_display_systems!($app, $state $(, $rest)+);
    }};
}

pub fn enter_display_system(mut commands: Commands, display: Res<InputDisplayRes>) {
    // Spawn each `AtomicDisplay` in the `InputDisplay` resource.
    for atom in display.atoms.iter() {
        match atom {
            TaggedAtomicParams::Button(params) => ButtonAtomicDisplay::spawn(&mut commands, params),
            TaggedAtomicParams::AnalogStick(params) => {
                AnalogStickAtomicDisplay::spawn(&mut commands, params)
            }
            TaggedAtomicParams::Frame(params) => FrameAtomicDisplay::spawn(&mut commands, params),
        }
    }
}

pub fn insert_display_from_file(mut commands: Commands, path: &str) {
    // Attempt to inject an input display from a file, and inject an empty display if that fails.
    match read_from_file::<InputDisplayRes>(path) {
        Ok(display) => {
            commands.insert_resource(display);
        }
        Err(e) => {
            println!("Error reading input display from file '{}': {:?}", path, e);
            commands.insert_resource(InputDisplayRes::default());
        }
    }
}

pub fn startup_display_system(mut commands: Commands) {
    // insert_display_from_file(commands, "display.json");
}

pub fn save_display_to_file(mut commands: Commands, display: Res<InputDisplayRes>) {
    write_to_file(display.into_inner(), "display.json");
}

pub fn add_display_systems(app: &mut App, display_state: AppState) {
    // Startup
    app.add_startup_system(startup_display_system);

    // Enter display state
    app.add_system_set(SystemSet::on_enter(display_state).with_system(enter_display_system));

    // Exit display state
    app.add_system_set(SystemSet::on_exit(display_state).with_system(save_display_to_file));

    // Atomic display-specific systems
    add_atomic_display_systems!(
        app,
        display_state,
        ButtonAtomicDisplay,
        AnalogStickAtomicDisplay,
        FrameAtomicDisplay
    );
}
