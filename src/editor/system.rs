use bevy::{prelude::*, utils::HashMap, window::WindowId};
use bevy_egui::{egui, EguiContext, EguiInput, EguiSystem};
use bevy_inspector_egui::WorldInspectorParams;

use crate::state::AppState;

use super::mouse::{
    editor_mouse_drag_system, editor_mouse_run_criteria, editor_mouse_scroll_system,
};

pub fn listen_egui_events(mut egui_input: ResMut<EguiContext>) {
    println!("found it: {:?}", egui_input.ctx_mut().wants_pointer_input());
}

pub fn add_editor_systems(app: &mut App, editor_state: AppState) {
    // app.add_system_set(
    //     SystemSet::on_enter(AppState::Editor)
    //         .with_system(startup)
    // );
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(editor_mouse_run_criteria)
            .with_system(editor_mouse_scroll_system)
            .with_system(editor_mouse_drag_system),
    );
}
