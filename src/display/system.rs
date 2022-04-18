use std::path::Path;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::RegisterInspectable;

use crate::{
    controller::layout::ControllerLayoutsRes,
    util::{read_from_file, write_to_file},
    AppState,
};

use super::{
    analog_stick::AnalogStickAtomicDisplay,
    button::ButtonAtomicDisplay,
    display::{
        AtomicDisplay, AtomicInputDisplay, InputDisplay, RootAtomicDisplayMarker,
        SerialInputDisplay, TaggedAtomicParams,
    },
    frame::FrameAtomicDisplay,
};

pub fn spawn_atomic_display(mut commands: &mut Commands, mut atom: &mut AtomicDisplay) {
    // Spawn entities for the parameters of `atom` and save a reference to the root spawned `Entity`.
    let entity = match *atom.params {
        TaggedAtomicParams::Button(b) => ButtonAtomicDisplay::spawn(&mut commands, &b),
        TaggedAtomicParams::AnalogStick(asp) => {
            AnalogStickAtomicDisplay::spawn(&mut commands, &asp)
        }
        TaggedAtomicParams::Frame(f) => FrameAtomicDisplay::spawn(&mut commands, &f),
    };

    // Record the root entity associated to `atom`.
    atom.entity = Some(entity);
}

pub struct RequestSpawnAtom(pub AtomicDisplay);

fn handle_request_spawn_atom_system(
    mut event_reader: EventReader<RequestSpawnAtom>,
    mut commands: Commands,
) {
    for RequestSpawnAtom(atom) in event_reader.iter() {
        spawn_atomic_display(&mut commands, &mut atom.clone());
    }
}

pub struct RequestDespawnAtom(pub Entity);

fn handle_request_despawn_atom_system(
    mut event_reader: EventReader<RequestDespawnAtom>,
    mut commands: Commands,
) {
    for &RequestDespawnAtom(entity) in event_reader.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct RequestDespawnAll;

fn handle_request_despawn_all_system(
    mut event_reader: EventReader<RequestDespawnAll>,
    mut commands: Commands,
    query: Query<Entity, With<RootAtomicDisplayMarker>>,
) {
    for _ in event_reader.iter() {
        // clear the `InputDisplay` resource
        commands.insert_resource(InputDisplay::default());

        // delete entities belonging to atomic displays
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        break;
    }
}

pub struct RequestSaveDisplay;

pub fn handle_request_save_display(
    mut event_reader: EventReader<RequestSaveDisplay>,
    display: Res<InputDisplay>,
) {
    for e in event_reader.iter() {
        let mut atoms = vec![];

        for atom in display.atoms.iter() {
            let x = *atom.params;
            println!("atom: {:?}", x);
            atoms.push(x);
        }

        let title = display.title.clone();
        let path = &format!("displays/{}.json", title)[..];
        let serial_display = SerialInputDisplay {
            atoms,
            title: display.title.clone(),
        };
        write_to_file(&serial_display, path);
    }
}
// pub fn insert_display_from_file(mut commands: Commands, path: &str) {
//     // Attempt to inject an input display from a file, and inject an empty display if that fails.
//     match read_from_file::<InputDisplay>(path) {
//         Ok(display) => {
//             commands.insert_resource(QueuedInputDisplay { display });
//         }
//         Err(e) => {
//             println!("Error reading input display from file '{}': {:?}", path, e);
//         }
//     }
// }

// pub fn save_display_to_file(mut commands: Commands, display: Res<InputDisplay>) {
//     write_to_file(display.into_inner(), "display.json");
// }

pub fn add_display_systems(app: &mut App) {
    // app.add_system(spawn_queued_display_system.after("teardown"));
    app.register_inspectable::<TaggedAtomicParams>();

    app.add_event::<RequestDespawnAtom>();
    app.add_system(handle_request_despawn_atom_system);

    app.add_event::<RequestSpawnAtom>();
    app.add_system(handle_request_spawn_atom_system);

    app.add_event::<RequestDespawnAll>();
    app.add_system(handle_request_despawn_all_system);

    app.add_event::<RequestSaveDisplay>();
    app.add_system(handle_request_save_display);

    app.insert_resource(InputDisplay::default());

    // Atomic display-specific systems
    ButtonAtomicDisplay::add_update_systems(app);
    AnalogStickAtomicDisplay::add_update_systems(app);
    FrameAtomicDisplay::add_update_systems(app);
}
