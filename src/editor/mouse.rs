use bevy::{
    ecs::schedule::ShouldRun,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContext;

use crate::{state::AppState, util::screen_to_world, MainCameraMarker};

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 5.0;
const LINE_SCROLL_SPEED: f32 = 0.1;
const PIXEL_SCROLL_SPEED: f32 = 0.04;
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
    let ctx = egui_input.ctx_mut();

    // Ignore mouse input if the cursor is over an egui window
    // or if egui is using the current mouse input.
    let is_egui_using_mouse = ctx.is_pointer_over_area() || ctx.wants_pointer_input();

    if *app_state.current() == AppState::Editor && !is_egui_using_mouse {
        return ShouldRun::Yes;
    }

    ShouldRun::No
}

// A system to release the mouse when the game window loses focus.
pub fn release_mouse_when_unfocused_system(
    mut mouse_buttons: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        if !window.is_focused() {
            // Release the mouse
            if mouse_buttons.pressed(MouseButton::Left) {
                mouse_buttons.release(MouseButton::Left);
            }
        }
    }
}

// A system to zoom the main camera in response to mouse scrolling.
pub fn editor_mouse_scroll_system(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<MainCameraMarker>>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let mut orth_proj = query.single_mut();
                orth_proj.scale -= LINE_SCROLL_SPEED * ev.y;
                orth_proj.scale = orth_proj.scale.clamp(MIN_ZOOM, MAX_ZOOM);
            }
            MouseScrollUnit::Pixel => {
                let mut orth_proj = query.single_mut();
                orth_proj.scale -= PIXEL_SCROLL_SPEED * ev.y;
                orth_proj.scale = orth_proj.scale.clamp(MIN_ZOOM, MAX_ZOOM);
            }
        }
    }
}

// A system to move the main camera in response to mouse dragging.
pub fn editor_mouse_drag_system(
    mouse_buttons: ResMut<Input<MouseButton>>,
    frozen_pos: Option<Res<FrozenCursorPos>>,
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut evr_motion: EventReader<MouseMotion>,
    mut query: Query<
        (&mut GlobalTransform, &OrthographicProjection, &Camera),
        With<MainCameraMarker>,
    >,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        // Mouse was just released: exit drag mode.
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
    } else if mouse_buttons.pressed(MouseButton::Left) {
        // Mouse is held down: update drag mode.
        let (mut transform, orth_proj, camera) = query.single_mut();

        if mouse_buttons.just_pressed(MouseButton::Left) {
            // Mouse was just pressed: enter drag mode.
            // Hide and lock the cursor while recording its world position.
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
            if let Some(screen_pos) = window.cursor_position() {
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
    } else {
    }
}
