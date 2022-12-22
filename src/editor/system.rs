use bevy::prelude::*;

use bevy_inspector_egui::WorldInspectorParams;

use crate::state::AppState;

use super::{
    mouse::{
        editor_mouse_drag_system, editor_mouse_run_criteria, editor_mouse_scroll_system,
        release_mouse_when_unfocused_system,
    },
    top_bar::{display_top_bar_system, top_bar_startup_system},
};

fn enter_editor_system(inspector_windows: Option<ResMut<WorldInspectorParams>>) {
    if let Some(mut params) = inspector_windows {
        params.enabled = true;
    }
}

fn exit_editor_system(inspector_windows: Option<ResMut<WorldInspectorParams>>) {
    if let Some(mut params) = inspector_windows {
        params.enabled = false;
    }
}

pub fn add_editor_systems(app: &mut App, editor_state: AppState) {
    // Enter editor state
    app.add_system_set(SystemSet::on_enter(editor_state).with_system(enter_editor_system));

    // Exit editor state
    app.add_system_set(SystemSet::on_exit(editor_state).with_system(exit_editor_system));

    // Fix mouse state when window is unfocused
    app.add_system(release_mouse_when_unfocused_system);

    // Update top bar in editor
    app.add_system_set(SystemSet::on_update(editor_state).with_system(display_top_bar_system));

    app.add_startup_system(top_bar_startup_system);

    // Editor mouse events rely on knowing where all the egui windows are,
    // so they need to be run after all egui stuff has been drawn (?? i think)
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_run_criteria(editor_mouse_run_criteria)
            .with_system(editor_mouse_scroll_system)
            .with_system(editor_mouse_drag_system),
    );
}
