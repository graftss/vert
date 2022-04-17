use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Mesh2dHandle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, StrokeMode},
    render::Shape,
    shapes::{self, Rectangle},
};
use serde::{Deserialize, Serialize};

use crate::util::despawn_all_with;

use super::{
    display::{AtomicInputDisplay, RootAtomicDisplayMarker, TaggedAtomicParams},
    renderable::Renderable,
    serialization::{RectangleDef, RegularPolygonDef},
};

const FRAME_Z_POS: f32 = 100.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable)]
pub struct FrameParams {
    #[inspectable(label = "Position")]
    pub position: Vec2,

    #[inspectable(label = "Height", min = 100.0, max = 600.0, suffix = "px")]
    pub height: f32,

    #[inspectable(label = "Width", min = 100.0, max = 800.0, suffix = "px")]
    pub width: f32,

    #[inspectable(label = "Thickness", min = 1.0, max = 10.0, suffix = "px")]
    pub thickness: f32,
}

impl FrameParams {
    fn root_bundle(self) -> impl Bundle {
        let name = "** Frame".to_string();
        (
            RootFrameMarker,
            RootAtomicDisplayMarker,
            GlobalTransform::identity(),
            Transform::from_xyz(self.position.x, self.position.y, FRAME_Z_POS),
            Name::new(name),
        )
    }

    fn insert_child_bundle(self, mut commands: EntityCommands) -> impl Bundle {
        let FrameParams {
            thickness,
            position,
            height,
            width,
        } = self;

        let extents = Vec2::new(width, height);
        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(Color::GREEN, thickness),
        };
        let transform = Transform::identity();

        commands.insert(ChildFrameMarker);
        Renderable::Rectangle(RectangleDef { extents }).insert_bundle(
            &mut commands,
            draw_mode,
            transform,
        );
    }
}

impl Default for FrameParams {
    fn default() -> Self {
        Self {
            position: Vec2::new(-100.0, -80.0),
            height: 160.0,
            width: 200.0,
            thickness: 3.0,
        }
    }
}

#[derive(Component)]
pub struct RootFrameMarker;

#[derive(Component)]
pub struct ChildFrameMarker;

pub struct FrameAtomicDisplay;

impl FrameAtomicDisplay {
    fn regenerate_system(
        mut commands: Commands,
        mut parent_query: Query<(Entity, &FrameParams, &Children), Changed<FrameParams>>,
        mut child_query: Query<&ChildFrameMarker>,
    ) {
        for (root_entity, params, children) in parent_query.iter_mut() {
            // Regenerate the root entity
            commands
                .entity(root_entity)
                .insert_bundle(params.root_bundle());

            for &child_entity in children.iter() {
                match child_query.get(child_entity) {
                    Ok(_) => {
                        params.insert_child_bundle(commands.entity(child_entity));
                    }
                    Err(_) => {
                        panic!("failed to regenerate frame");
                    }
                }
            }
            // params.insert_child_bundle(commands.entity(entity));
        }
    }
}

impl AtomicInputDisplay<FrameParams> for FrameAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &FrameParams) -> Entity {
        let mut root_entity = commands.spawn();
        let my_params = params.clone();

        let id = root_entity
            .insert(RootFrameMarker)
            .insert(RootAtomicDisplayMarker)
            .insert(Name::new("** Frame"))
            .insert(my_params)
            .id();

        root_entity.with_children(|parent| {
            params.insert_child_bundle(parent.spawn());
        });

        id
    }

    fn add_update_systems(app: &mut App) {
        app.register_inspectable::<FrameParams>();
        app.add_system(FrameAtomicDisplay::regenerate_system);
    }
}
