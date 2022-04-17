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
    display::{AtomicInputDisplay, RootAtomicDisplayMarker},
    renderable::Renderable,
    serialization::{RectangleDef, RegularPolygonDef},
};

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
    fn bundle(self) -> impl Bundle {
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
        let transform = Transform::from_xyz(position.x, position.y, 100.0);

        Renderable::Rectangle(RectangleDef { extents }).build_as(draw_mode, transform)
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

pub struct FrameAtomicDisplay;

impl FrameAtomicDisplay {
    fn regenerate_system(
        mut commands: Commands,
        mut query: Query<(Entity, &FrameParams), Changed<FrameParams>>,
    ) {
        for (entity, params) in query.iter_mut() {
            commands.entity(entity).insert_bundle(params.bundle());
        }
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
        app.add_system(FrameAtomicDisplay::regenerate_system);
    }
}
