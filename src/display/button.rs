use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    controller::layout::ControllerKey,
    editor::inspector::{BoundControllerKey, InputSinkId},
    input::input::{InputSink, InputValue},
    state::AppState,
    util::despawn_all_with,
};

use super::{
    display::{AtomicInputDisplay, Renderable, RootAtomicDisplayMarker},
    serialization::{DrawModeDef, TransformDef},
};

// The data parameterizing a button input display.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable)]
pub struct ButtonParams {
    #[inspectable(ignore)]
    pub displayable: Renderable,
    #[inspectable(ignore)]
    pub on_mode: DrawModeDef,
    #[inspectable(ignore)]
    pub off_mode: DrawModeDef,
    pub transform: TransformDef,
    pub button_key: BoundControllerKey,
}

impl ButtonParams {
    pub fn root_bundle(self) -> impl Bundle {
        let name = format!("Button ({})", self.button_key.key.to_string());
        (
            GlobalTransform::identity(),
            Into::<Transform>::into(self.transform),
            RootButtonMarker,
            RootAtomicDisplayMarker,
            Name::new(name),
        )
    }

    pub fn insert_on_bundle(&self, mut commands: EntityCommands) {
        let mut on_bundle = self
            .displayable
            .build_as(self.on_mode.into(), Transform::identity());
        on_bundle.visibility = Visibility { is_visible: false };
        commands
            .insert_bundle(on_bundle)
            .insert(ChildButtonMarker { pressed: true })
            .insert(InputSink::new(vec![self.button_key.key]));
    }

    fn insert_off_bundle(&self, mut commands: EntityCommands) {
        let mut off_bundle = self
            .displayable
            .build_as(self.off_mode.into(), Transform::identity());

        commands
            .insert_bundle(off_bundle)
            .insert(ChildButtonMarker { pressed: false })
            .insert(InputSink::new(vec![self.button_key.key]));
    }
}

// The marker for the root entity of a button display.
#[derive(Component)]
pub struct RootButtonMarker;

// The marker for a child entity of a button display.
// The root display has two children whose visibilities are toggled
// according to the button input.
#[derive(Component)]
pub struct ChildButtonMarker {
    // The button press state for which this child entity is visible.
    pub pressed: bool,
}

pub struct ButtonAtomicDisplay;

impl ButtonAtomicDisplay {
    // Update all atomic button displays.
    fn button_update_system(mut query: Query<(&InputSink, &ChildButtonMarker, &mut Visibility)>) {
        for (sink, marker, mut vis) in query.iter_mut() {
            match sink.values[0] {
                Some(InputValue::Button(true)) => {
                    vis.is_visible = marker.pressed;
                }
                Some(InputValue::Button(false)) => {
                    vis.is_visible = !marker.pressed;
                }
                _ => {
                    vis.is_visible = false;
                }
            }
        }
    }

    fn regenerate_system(
        mut commands: Commands,
        parent_query: Query<(Entity, &ButtonParams, &Children), Changed<ButtonParams>>,
        child_query: Query<&ChildButtonMarker>,
    ) {
        for (root_entity, params, children) in parent_query.iter() {
            // Regenerate the root entity
            commands
                .entity(root_entity)
                .insert_bundle(params.root_bundle());

            // Rengenerate the child entities
            for &child_entity in children.iter() {
                let marker_result = child_query.get(child_entity);
                match marker_result {
                    Ok(marker) => {
                        if marker.pressed {
                            params.insert_on_bundle(commands.entity(child_entity));
                        } else {
                            params.insert_off_bundle(commands.entity(child_entity));
                        }
                    }
                    _ => {
                        panic!("weird thing happened when regenerating a button");
                    }
                }
            }
        }
    }
}

impl AtomicInputDisplay<ButtonParams> for ButtonAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &ButtonParams) -> Entity {
        let mut root = commands.spawn_bundle(params.root_bundle());

        let mut my_params = *params;
        my_params.button_key.id = Some(InputSinkId {
            entity: Some(root.id()),
            idx: 0,
        });

        root.insert(my_params)
            .with_children(|parent| {
                params.insert_on_bundle(parent.spawn());
                params.insert_off_bundle(parent.spawn());
            })
            .id()
    }

    fn add_update_systems(app: &mut App) {
        app.add_system(Self::button_update_system);
        app.add_system(Self::regenerate_system);
        app.register_inspectable::<ButtonParams>();
    }
}
