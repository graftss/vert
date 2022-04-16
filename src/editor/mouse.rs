use bevy::{
    ecs::schedule::ShouldRun,
    input::mouse::{MouseMotion, MouseWheel},
    pbr::VisiblePointLights,
    prelude::*,
    render::camera::CameraProjection,
};
use bevy_egui::EguiContext;

use crate::{state::AppState, util::screen_to_world, MainCameraMarker};

const FIXED_SCROLL_SPEED: f32 = 0.1;
const FIXED_DRAG_SPEED: f32 = 0.8;

// A record of the world position when the cursor was frozen and hidden during an editor drag.
#[derive(Debug)]
pub struct FrozenCursorPos {
    world_pos: Vec2,
}

pub fn editor_mouse_run_criteria(
    app_state: Res<State<AppState>>,
    mut egui_input: ResMut<EguiContext>,
) -> ShouldRun {
    if *app_state.current() == AppState::Editor && !egui_input.ctx_mut().is_pointer_over_area() {
        return ShouldRun::Yes;
    }

    ShouldRun::No
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
    mut frozen_pos: Option<Res<FrozenCursorPos>>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut evr_motion: EventReader<MouseMotion>,
    mut query: Query<
        (&mut GlobalTransform, &OrthographicProjection, &Camera),
        With<MainCameraMarker>,
    >,
) {
    if mouse_buttons.pressed(MouseButton::Left) {
        // Mouse is held down.
        let (mut transform, orth_proj, camera) = query.single_mut();

        if mouse_buttons.just_pressed(MouseButton::Left) {
            // If the mouse was just pressed, hide and lock the cursor while recording its world position.
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
            if let Some(mut screen_pos) = window.cursor_position() {
                let world_pos = screen_to_world(&transform, &camera, &window, &screen_pos);
                commands.insert_resource(FrozenCursorPos { world_pos });
            }
        } else {
            // If the mouse was already pressed, translate the camera according to mouse movement.
            for ev in evr_motion.iter() {
                transform.translation.x -= ev.delta.x * FIXED_DRAG_SPEED * orth_proj.scale;
                transform.translation.y += ev.delta.y * FIXED_DRAG_SPEED * orth_proj.scale;
            }
        }
    } else if mouse_buttons.just_released(MouseButton::Left) {
        // Mouse was just released.
        // If we have recorded a world position of the cursor, restore it (by moving the cursor there).
        let new_cursor_pos = frozen_pos.map(|frozen| {
            let (transform, _, camera) = query.single_mut();
            camera
                .world_to_screen(&windows, &transform, frozen.world_pos.extend(1.0))
                .unwrap()
        });

        if let Some(window) = windows.get_primary_mut() {
            // Restore the world position of the cursor if one exists.
            if let Some(pos) = new_cursor_pos {
                window.set_cursor_position(pos);
                commands.remove_resource::<FrozenCursorPos>();
            }

            // Restore normal movement/visibility of the cursor.
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }
    }
}
