use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, StrokeMode},
    render::Shape,
    shapes::{self, Rectangle},
};
use serde::{Deserialize, Serialize};

use crate::util::despawn_all_with;

use super::{
    display::{AtomicInputDisplay, Renderable, RootAtomicDisplayMarker},
    serialization::{RectangleDef, RegularPolygonDef},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable)]
pub struct FrameParams {
    pub left: f32,
    pub bottom: f32,

    #[inspectable(min = 100.0, max = 600.0, suffix = "px")]
    pub height: f32,

    #[inspectable(min = 100.0, max = 800.0, suffix = "px")]
    pub width: f32,

    #[inspectable(min = 1.0, max = 10.0, suffix = "px")]
    pub thickness: f32,
}

impl FrameParams {
    pub fn bundle(self) -> impl Bundle {
        let FrameParams {
            thickness,
            left,
            bottom,
            height,
            width,
        } = self;

        let extents = Vec2::new(width, height);
        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(Color::GREEN, thickness),
        };
        let transform = Transform::from_xyz(left, bottom, 100.0);

        Renderable::Rectangle(RectangleDef { extents }).build_as(draw_mode, transform)
    }
}

impl Default for FrameParams {
    fn default() -> Self {
        Self {
            left: -100.0,
            bottom: -80.0,
            height: 160.0,
            width: 200.0,
            thickness: 3.0,
        }
    }
}

#[derive(Component)]
pub struct RootFrameMarker;

pub struct FrameAtomicDisplay;

fn regenerate_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &FrameParams, &DrawMode, &mut Transform, &mut Shape),
        Changed<FrameParams>,
    >,
) {
    for (entity, params, draw_mode, mut transform, mut shape) in query.iter_mut() {
        commands.entity(entity).insert_bundle(params.bundle());
    }
}

impl AtomicInputDisplay<FrameParams> for FrameAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &FrameParams) -> Entity {
        let my_params = params.clone();
        let frame_bundle = params.bundle();

        commands
            .spawn_bundle(frame_bundle)
            .insert(RootFrameMarker)
            .insert(RootAtomicDisplayMarker)
            .insert(Name::new("Frame"))
            .insert(my_params)
            .id()
    }

    fn add_update_systems(app: &mut App) {
        app.register_inspectable::<FrameParams>();
        app.add_system(regenerate_system);
    }
}
