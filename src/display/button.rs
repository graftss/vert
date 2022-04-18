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
    display::{AtomicInputDisplay, RootAtomicDisplayMarker, TaggedAtomicParams},
    renderable::Renderable,
    serialization::{DrawModeDef, TransformDef},
};

// The data parameterizing a button input display.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable, Default)]
pub struct ButtonParams {
    #[inspectable(label = "Button")]
    pub button_key: BoundControllerKey,
    #[inspectable(label = "Transform")]
    pub transform: TransformDef,
    #[inspectable(label = "Model")]
    pub displayable: Renderable,
    #[inspectable(label = "On texture")]
    pub on_mode: DrawModeDef,
    #[inspectable(label = "Off texture")]
    pub off_mode: DrawModeDef,
}

impl ButtonParams {
    pub fn root_bundle(self) -> impl Bundle {
        let name = "** Button".to_string();
        (
            GlobalTransform::identity(),
            Into::<Transform>::into(self.transform),
            RootButtonMarker,
            RootAtomicDisplayMarker,
            Name::new(name),
        )
    }

    pub fn insert_on_bundle(&self, mut commands: &mut EntityCommands) {
        // Insert the model and texture bundles
        self.displayable
            .insert_bundle(commands, self.on_mode.into(), Transform::identity());

        commands
            .insert(Visibility { is_visible: false })
            .insert(ChildButtonMarker { pressed: true })
            .insert(InputSink::new(vec![self.button_key.key]));
    }

    fn insert_off_bundle(&self, mut commands: &mut EntityCommands) {
        self.displayable
            .insert_bundle(commands, self.off_mode.into(), Transform::identity());

        commands
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
        parent_query: Query<
            (Entity, &TaggedAtomicParams, &Children),
            (With<RootButtonMarker>, Changed<TaggedAtomicParams>),
        >,
        child_query: Query<&ChildButtonMarker>,
    ) {
        for (root_entity, tagged_params, children) in parent_query.iter() {
            if let TaggedAtomicParams::Button(params) = tagged_params {
                // Regenerate the root entity
                commands
                    .entity(root_entity)
                    .insert_bundle(params.root_bundle());

                // Rengenerate the child entities
                for &child_entity in children.iter() {
                    match child_query.get(child_entity) {
                        Ok(marker) => {
                            if marker.pressed {
                                params.insert_on_bundle(&mut commands.entity(child_entity));
                            } else {
                                params.insert_off_bundle(&mut commands.entity(child_entity));
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
}

impl AtomicInputDisplay<ButtonParams> for ButtonAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &ButtonParams) -> Entity {
        let mut root = commands.spawn_bundle(params.root_bundle());

        let mut my_params = *params;
        my_params.button_key.bind(root.id(), 0);

        root.insert(TaggedAtomicParams::Button(my_params))
            .with_children(|parent| {
                params.insert_on_bundle(&mut parent.spawn());
                params.insert_off_bundle(&mut parent.spawn());
            })
            .id()
    }

    fn add_update_systems(app: &mut App) {
        app.add_system(Self::button_update_system);
        app.add_system(Self::regenerate_system);
        app.register_inspectable::<ButtonParams>();
    }
}
