use bevy::{prelude::*, utils::HashMap, window::WindowId};
use bevy_egui::{egui, EguiContext, EguiInput, EguiSystem};
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorParams};

use crate::{display::display::InputDisplay, state::AppState};

use super::{
    inspector::display_inspector_system,
    mouse::{editor_mouse_drag_system, editor_mouse_run_criteria, editor_mouse_scroll_system},
};

pub fn listen_egui_events(mut egui_input: ResMut<EguiContext>) {
    println!("found it: {:?}", egui_input.ctx_mut().wants_pointer_input());
}

pub struct TestState {
    pub num: i32,
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

    app.insert_resource(TestState { num: 10 });

    app.add_system_set(SystemSet::on_update(editor_state).with_system(display_inspector_system));
}
