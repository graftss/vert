use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;

use crate::{
    editor::top_bar::TopBarState,
    util::{read_from_file, write_to_file},
};

use super::{
    analog_stick::{AnalogStickAtomicDisplay, AnalogStickParams},
    button::{ButtonAtomicDisplay, ButtonParams},
    display::{
        AtomicDisplay, AtomicInputDisplay, AtomicParamsTag, InputDisplay, InputDisplayMetadata,
        RootAtomicDisplayMarker, SerialInputDisplay, TaggedAtomicParams,
    },
    frame::FrameAtomicDisplay,
};

pub fn spawn_atomic_display(mut commands: &mut Commands, mut atom: &mut AtomicDisplay) {
    // Spawn entities for the parameters of `atom` and save a reference to the root spawned `Entity`.
    let entity = match atom.params.as_ref() {
        TaggedAtomicParams::Button(b) => ButtonAtomicDisplay::spawn(&mut commands, &b),
        TaggedAtomicParams::AnalogStick(asp) => {
            AnalogStickAtomicDisplay::spawn(&mut commands, &asp)
        }
        TaggedAtomicParams::Frame(f) => FrameAtomicDisplay::spawn(&mut commands, &f),
    };

    // Record the root entity associated to `atom`.
    atom.entity = Some(entity);
}

#[derive(Debug, Clone)]
pub enum RequestSpawnAtom {
    // Spawn an existing atom from its params.
    Existing(AtomicDisplay),
    // Spawn a new atom of the given type at the given position.
    New(AtomicParamsTag, Vec2),
}

fn handle_request_spawn_atom_system(
    mut event_reader: EventReader<RequestSpawnAtom>,
    mut commands: Commands,
) {
    for e in event_reader.iter() {
        match e {
            RequestSpawnAtom::Existing(atom) => {
                let mut x = atom.clone();
                spawn_atomic_display(&mut commands, &mut x);
            }
            &RequestSpawnAtom::New(tag, pos) => {
                let entity = None;
                let params = match tag {
                    AtomicParamsTag::Button => Box::new(TaggedAtomicParams::Button(ButtonParams {
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0).into(),
                        ..Default::default()
                    })),
                    AtomicParamsTag::AnalogStick => {
                        Box::new(TaggedAtomicParams::AnalogStick(AnalogStickParams {
                            transform: Transform::from_xyz(pos.x, pos.y, 0.0).into(),
                            ..Default::default()
                        }))
                    }
                };

                spawn_atomic_display(&mut commands, &mut AtomicDisplay { params, entity })
            }
            _ => {}
        }
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
    query: Query<&TaggedAtomicParams>,
    top_bar_state: Res<TopBarState>,
) {
    for _e in event_reader.iter() {
        let mut atoms = vec![];

        for atom in query.iter() {
            atoms.push(atom.clone());
        }

        let display_name = top_bar_state.display_name.clone();
        let path = format!("displays/{}.json", display_name);
        let metadata = InputDisplayMetadata {
            title: display_name,
        };
        let serial_display = SerialInputDisplay { atoms, metadata };
        write_to_file(&serial_display, &path);
    }
}

pub struct RequestLoadDisplay(pub String);

pub fn handle_request_load_display(
    mut commands: Commands,
    mut query: Query<Entity, With<RootAtomicDisplayMarker>>,
    mut er_reqload: EventReader<RequestLoadDisplay>,
    mut ew_spawn: EventWriter<RequestSpawnAtom>,
    mut top_bar_state: ResMut<TopBarState>,
) {
    for RequestLoadDisplay(path) in er_reqload.iter() {
        match read_from_file::<SerialInputDisplay>(&path) {
            Ok(display) => {
                // Clear the current display
                for atom_entity in query.iter_mut() {
                    commands.entity(atom_entity).despawn_recursive();
                }

                // Spawn the requested display
                for params in display.atoms {
                    ew_spawn.send(RequestSpawnAtom::Existing(AtomicDisplay {
                        params: Box::new(params),
                        entity: None,
                    }));
                }

                top_bar_state.display_name = path[9..path.len() - 5].to_string();
            }
            Err(e) => {
                println!("Error reading input display from file '{}': {:?}", path, e);
            }
        }
    }
}

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

    app.add_event::<RequestLoadDisplay>();
    app.add_system(handle_request_load_display);

    app.insert_resource(InputDisplay::default());

    // Atomic display-specific systems
    ButtonAtomicDisplay::add_update_systems(app);
    AnalogStickAtomicDisplay::add_update_systems(app);
    FrameAtomicDisplay::add_update_systems(app);
}
