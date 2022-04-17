use bevy::{prelude::*, utils::HashMap, window::WindowId};
use bevy_egui::{egui, EguiContext, EguiInput, EguiSystem};
use bevy_inspector_egui::{
    plugin::InspectorWindows, InspectorPlugin, RegisterInspectable, WorldInspectorParams,
    WorldInspectorPlugin,
};

use crate::{
    display::display::{InputDisplay, TaggedAtomicParams},
    state::AppState,
};

use super::{mouse::{
    editor_mouse_drag_system, editor_mouse_run_criteria, editor_mouse_scroll_system,
    release_mouse_when_unfocused_system,
}, display_fs::display_fs_system};

fn enter_editor_system(mut inspector_windows: Option<ResMut<WorldInspectorParams>>) {
    if let Some(mut params) = inspector_windows {
        params.enabled = true;
    }
}

fn exit_editor_system(mut inspector_windows: Option<ResMut<WorldInspectorParams>>) {
    if let Some(mut params) = inspector_windows {
        params.enabled = false;
    }
}

pub fn add_editor_systems(app: &mut App, editor_state: AppState) {
    // Enter editor state
    app.add_system_set(SystemSet::on_enter(AppState::Editor).with_system(enter_editor_system));

    // Exit editor state
    app.add_system_set(SystemSet::on_exit(AppState::Editor).with_system(exit_editor_system));

    app.add_system(release_mouse_when_unfocused_system);

    app.add_system_set(
        SystemSet::on_update(editor_state)
            .with_system(display_fs_system)
    );

    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(editor_mouse_run_criteria)
            .with_system(editor_mouse_scroll_system)
            .with_system(editor_mouse_drag_system),
    );
}
