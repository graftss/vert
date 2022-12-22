use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, StrokeMode};
use serde::{Deserialize, Serialize};

use crate::util::invert_color;

use super::{
    display::{AtomicInputDisplay, RootAtomicDisplayMarker, TaggedAtomicParams},
    renderable::Renderable,
    serialization::RectangleDef,
};

const FRAME_Z_POS: f32 = 0.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable)]
pub struct FrameParams {
    #[inspectable(label = "Position")]
    pub position: Vec2,

    #[inspectable(label = "Height", min = 100.0, max = 600.0, suffix = "px")]
    pub height: f32,

    #[inspectable(label = "Width", min = 100.0, max = 800.0, suffix = "px")]
    pub width: f32,

    #[inspectable(label = "Background", alpha = true)]
    pub color: Color,

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
            position: _,
            height,
            width,
            color,
        } = self;

        let extents = Vec2::new(width, height);
        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(invert_color(color), thickness),
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
            color: Color::NONE,
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
        mut parent_query: Query<
            (Entity, &TaggedAtomicParams, &Children),
            (With<RootFrameMarker>, Changed<TaggedAtomicParams>),
        >,
        child_query: Query<&ChildFrameMarker>,
    ) {
        for (root_entity, tagged_params, children) in parent_query.iter_mut() {
            if let TaggedAtomicParams::Frame(params) = tagged_params {
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
            }
        }
    }
}

impl AtomicInputDisplay<FrameParams> for FrameAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &FrameParams) -> Entity {
        let mut root_entity = commands.spawn();

        let id = root_entity
            .insert_bundle(params.root_bundle())
            .insert(TaggedAtomicParams::Frame(params.clone()))
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
