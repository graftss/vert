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
        AtomicDisplay, AtomicInputDisplay, InputDisplay, QueuedInputDisplay,
        RootAtomicDisplayMarker, TaggedAtomicParams,
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

pub fn on_queued_display(queued_event: Option<Res<QueuedInputDisplay>>) -> ShouldRun {
    match queued_event {
        Some(_) => ShouldRun::Yes,
        None => ShouldRun::No,
    }
}

pub fn spawn_atomic_display(mut commands: &mut Commands, mut atom: &mut AtomicDisplay) {
    // Spawn entities for the parameters of `atom` and save a reference to the root spawned `Entity`.
    let entity = match atom.params {
        TaggedAtomicParams::Button(b) => ButtonAtomicDisplay::spawn(&mut commands, &b),
        TaggedAtomicParams::AnalogStick(asp) => {
            AnalogStickAtomicDisplay::spawn(&mut commands, &asp)
        }
        TaggedAtomicParams::Frame(f) => FrameAtomicDisplay::spawn(&mut commands, &f),
    };

    // Record the root entity associated to `atom`.
    atom.entity = Some(entity);
}

// Spawn the `QueuedInputDisplayRes` resource as an input display,
// then move it to the `InputDisplayRes` resource.
pub fn spawn_queued_display_system(
    mut commands: Commands,
    mut queued_display_res: Option<ResMut<QueuedInputDisplay>>,
) {
    if let Some(mut queued) = queued_display_res {
        for mut atom in queued.display.atoms.iter_mut() {
            spawn_atomic_display(&mut commands, &mut atom);
        }

        commands.insert_resource(queued.display.to_owned());
        commands.remove_resource::<QueuedInputDisplay>();
    }
}

pub struct RequestDespawnAtom(pub usize);

fn handle_request_despawn_atom_system(
    mut event_reader: EventReader<RequestDespawnAtom>,
    mut commands: Commands,
    mut display_res: Option<ResMut<InputDisplay>>,
) {
    if let Some(mut display) = display_res {
        for &RequestDespawnAtom(atom_idx) in event_reader.iter() {
            let atom = display.atoms[atom_idx];
            if let Some(entity) = atom.entity {
                commands.entity(entity).despawn_recursive();
                display.atoms.remove(atom_idx);
            }
        }
    }
}

pub fn insert_display_from_file(mut commands: Commands, path: &str) {
    // Attempt to inject an input display from a file, and inject an empty display if that fails.
    match read_from_file::<InputDisplay>(path) {
        Ok(display) => {
            commands.insert_resource(QueuedInputDisplay { display });
        }
        Err(e) => {
            println!("Error reading input display from file '{}': {:?}", path, e);
        }
    }
}

pub fn save_display_to_file(mut commands: Commands, display: Res<InputDisplay>) {
    write_to_file(display.into_inner(), "display.json");
}

pub fn add_display_systems(app: &mut App) {
    app.add_event::<RequestDespawnAtom>();
    app.add_system(spawn_queued_display_system.after("teardown"));

    app.add_system(handle_request_despawn_atom_system);

    // Atomic display-specific systems
    add_atomic_display_systems!(
        app,
        ButtonAtomicDisplay,
        AnalogStickAtomicDisplay,
        FrameAtomicDisplay
    );
}
