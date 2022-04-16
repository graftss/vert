use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    pbr::VisiblePointLights,
    prelude::*,
    render::camera::CameraProjection,
};

use crate::{app_state::AppState, MainCameraMarker};

const FIXED_SCROLL_SPEED: f32 = 0.1;
const FIXED_DRAG_SPEED: f32 = 0.8;

// A record of the screen position when it was frozen and hidden during an editor drag.
#[derive(Debug)]
pub struct FrozenCursorPos {
    pos: Vec2,
}

pub fn editor_mouse_scroll_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut OrthographicProjection), With<MainCameraMarker>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut orth_proj = query.single_mut();
                orth_proj.scale -= FIXED_SCROLL_SPEED * ev.y;
            }
            MouseScrollUnit::Pixel => {
                todo!();
            }
        }
    }
}

pub fn editor_mouse_drag_system(
    mouse_buttons: Res<Input<MouseButton>>,
    frozen_pos: Option<Res<FrozenCursorPos>>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut evr_motion: EventReader<MouseMotion>,
    mut query: Query<
        (
            &mut Transform,
            &OrthographicProjection,
            &GlobalTransform,
            &Camera,
        ),
        With<MainCameraMarker>,
    >,
) {
    if mouse_buttons.pressed(MouseButton::Left) {
        let (mut transform, orth_proj, _, _) = query.single_mut();

        if mouse_buttons.just_pressed(MouseButton::Left) {
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
            if let Some(pos) = window.cursor_position() {
                commands.insert_resource(FrozenCursorPos { pos });
            }
        } else {
            for ev in evr_motion.iter() {
                transform.translation.x -= ev.delta.x * FIXED_DRAG_SPEED * orth_proj.scale;
                transform.translation.y += ev.delta.y * FIXED_DRAG_SPEED * orth_proj.scale;
            }
        }
    } else if mouse_buttons.just_released(MouseButton::Left) {
        let (mut transform, orth_proj, gt, camera) = query.single_mut();
        if let Some(window) = windows.get_primary_mut() {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
            if let Some(frozen_pos) = frozen_pos {
                window.set_cursor_position(frozen_pos.pos);
            }
        }
    }
}

pub fn add_editor_systems(app: &mut App, editor_state: AppState) {
    // app.add_system_set(
    //     SystemSet::on_enter(AppState::Editor)
    // );
    app.add_system_set(
        SystemSet::on_update(editor_state)
            .with_system(editor_mouse_scroll_system)
            .with_system(editor_mouse_drag_system),
    );
}
