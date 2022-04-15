use bevy::prelude::*;

use crate::{controller::layout::ControllerLayoutsRes, AppState};

use super::{
    analog_stick::AnalogStickAtomicDisplay,
    button::ButtonAtomicDisplay,
    display::{AtomicInputDisplay, InputDisplay, TaggedAtomicParams},
};

// Call the update and teardown systems for `atomic_type`.
// (`atomic_type` should implement `AtomicInputDisplay`).
macro_rules! add_systems {
    ($atomic_type:ty, $app:ident, $state:ident) => {
        <$atomic_type>::add_update_systems($app, $state);
        <$atomic_type>::add_teardown_systems($app, $state);
    };
}

// Call the update and teardown systems for a list of atomic types.
macro_rules! add_display_systems {
    ($app:ident, $state:ident, $atomic_type:ty) => { add_systems!($atomic_type, $app, $state); };

    ($app:ident, $state:ident, $atomic_type:ty $(, $rest:ty)+) => {{
        add_display_systems!($app, $state, $atomic_type);
        add_display_systems!($app, $state $(, $rest)+);
    }};
}

pub fn display_startup_system(mut commands: Commands, display: Res<InputDisplay>) {
    // Spawn each `AtomicDisplay` in the `InputDisplay` resource.
    for atom in display.atoms.iter() {
        match atom {
            TaggedAtomicParams::Button(params) => ButtonAtomicDisplay::spawn(&mut commands, params),
            TaggedAtomicParams::AnalogStick(params) => {
                AnalogStickAtomicDisplay::spawn(&mut commands, params)
            }
        }
    }
}

pub fn add_display_systems(app: &mut App, display_state: AppState) {
    // Startup
    app.add_system_set(SystemSet::on_enter(display_state).with_system(display_startup_system));

    add_display_systems!(
        app,
        display_state,
        ButtonAtomicDisplay,
        AnalogStickAtomicDisplay
    );
}
