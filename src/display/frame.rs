use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode, StrokeMode};
use serde::{Deserialize, Serialize};

use crate::util::despawn_all_with;

use super::{
    display::{AtomicInputDisplay, Renderable, RootAtomicDisplayMarker},
    serialization::{RectangleDef, RegularPolygonDef},
    system::on_queued_display,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrameParams {
    pub left: f32,
    pub bottom: f32,
    pub height: f32,
    pub width: f32,
    pub thickness: f32,
}

#[derive(Component)]
pub struct RootFrameMarker;

pub struct FrameAtomicDisplay;

impl AtomicInputDisplay<FrameParams> for FrameAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &FrameParams) {
        let FrameParams {
            thickness,
            left,
            bottom,
            height,
            width,
        } = *params;

        let extents = Vec2::new(width, height);
        let draw_mode = DrawMode::Outlined {
            fill_mode: FillMode::color(Color::NONE),
            outline_mode: StrokeMode::new(Color::GREEN, thickness),
        };
        let transform = Transform::from_xyz(left, bottom, 100.0);

        let frame_bundle =
            Renderable::Rectangle(RectangleDef { extents }).build_as(draw_mode, transform);

        commands
            .spawn_bundle(frame_bundle)
            .insert(RootFrameMarker)
            .insert(RootAtomicDisplayMarker)
            .insert(Name::new("Frame"));
    }

    fn add_update_systems(app: &mut App) {}

    fn add_teardown_systems(app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(on_queued_display)
                .with_system(despawn_all_with::<RootFrameMarker>)
                .label("teardown"),
        );
    }
}
