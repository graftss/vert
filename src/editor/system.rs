use bevy::{input::mouse::MouseWheel, pbr::VisiblePointLights, prelude::*};

use crate::app_state::AppState;

pub fn editor_mouse_input_system(
    mut evr_scroll: EventReader<MouseWheel>,
    query: Query<(&mut OrthographicProjection, &mut Transform), With<VisiblePointLights>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for ev in evr_scroll.iter() {
        match ev.unit {
            MouseScrollUnit::Line => println!("line {} {}", ev.y, ev.x),
            MouseScrollUnit::Pixel => println!("pixel {} {}", ev.y, ev.x),
        }
    }
}

pub fn add_editor_systems(app: &mut App, editor_state: AppState) {
    // app.add_system_set(
    //     SystemSet::on_enter(AppState::Editor)
    // );
    app.add_system_set(SystemSet::on_update(editor_state).with_system(editor_mouse_input_system));
}
