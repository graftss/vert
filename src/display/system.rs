use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{
    controller::layout::ControllerLayoutsRes,
    util::{read_from_file, write_to_file},
    AppState,
};

use super::{
    analog_stick::AnalogStickAtomicDisplay,
    button::ButtonAtomicDisplay,
    display::{
        AtomicInputDisplay, InputDisplayRes, QueuedInputDisplayRes, RootAtomicDisplayMarker,
        TaggedAtomicParams,
    },
    frame::FrameAtomicDisplay,
};

// Call the update and teardown systems for a list of atomic types.
macro_rules! add_atomic_display_systems {
    ($app:ident, $atomic_type:ty) => {
        <$atomic_type>::add_update_systems($app);
        <$atomic_type>::add_teardown_systems($app);
    };

    ($app:ident, $atomic_type:ty $(, $rest:ty)+) => {{
        add_atomic_display_systems!($app, $atomic_type);
        add_atomic_display_systems!($app $(, $rest)+);
    }};
}

pub fn on_queued_display(queued_event: Option<Res<QueuedInputDisplayRes>>) -> ShouldRun {
    match queued_event {
        Some(_) => ShouldRun::Yes,
        None => ShouldRun::No,
    }
}

pub fn spawn_atomic_display(mut commands: &mut Commands, atom: &TaggedAtomicParams) {
    match atom {
        TaggedAtomicParams::Button(b) => ButtonAtomicDisplay::spawn(&mut commands, &b),
        TaggedAtomicParams::AnalogStick(asp) => {
            AnalogStickAtomicDisplay::spawn(&mut commands, &asp)
        }
        TaggedAtomicParams::Frame(f) => FrameAtomicDisplay::spawn(&mut commands, &f),
    }
}

// Spawn the `QueuedInputDisplayRes` resource as an input display,
// then move it to the `InputDisplayRes` resource.
pub fn spawn_queued_display_system(
    mut commands: Commands,
    queued_display_res: Option<Res<QueuedInputDisplayRes>>,
) {
    if let Some(queued_display) = queued_display_res {
        for atom in queued_display.display.atoms.iter() {
            spawn_atomic_display(&mut commands, &atom);
        }

        commands.insert_resource(queued_display.display.to_owned());
        commands.remove_resource::<QueuedInputDisplayRes>();
    }
}

pub fn insert_display_from_file(mut commands: Commands, path: &str) {
    // Attempt to inject an input display from a file, and inject an empty display if that fails.
    match read_from_file::<InputDisplayRes>(path) {
        Ok(display) => {
            commands.insert_resource(QueuedInputDisplayRes { display });
        }
        Err(e) => {
            println!("Error reading input display from file '{}': {:?}", path, e);
        }
    }
}

pub fn save_display_to_file(mut commands: Commands, display: Res<InputDisplayRes>) {
    write_to_file(display.into_inner(), "display.json");
}

pub fn add_display_systems(app: &mut App) {
    app.add_system(spawn_queued_display_system.after("teardown"));

    // Atomic display-specific systems
    add_atomic_display_systems!(
        app,
        ButtonAtomicDisplay,
        AnalogStickAtomicDisplay,
        FrameAtomicDisplay
    );
}
