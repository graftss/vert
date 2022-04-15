use bevy::{prelude::*, utils::HashMap};
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;

use crate::{
    app_state::AppState,
    input::{input::InputSource, raw_input_reader::RawInputRes, RawInputReader},
    util::despawn_all_with,
};

use super::{
    layout::{ControllerLayout, ControllerLayoutRes, Ps2Key, Ps2Layout, PS2_KEY_ORDER},
    listener::{input_listener_system, InputListener},
};

pub fn startup(mut commands: Commands) {
    commands.insert_resource(ControllerLayoutRes {
        sources: HashMap::default(),
    });

    commands.insert_resource(InputListener::default());
}

const NO_BINDING: &'static str = "-";
const LISTEN_FOR_BINDING: &'static str = "Listening...";
const CONTROLLER_WINDOW_TITLE: &'static str = "Controller";

pub fn update(
    mut egui_ctx: ResMut<EguiContext>,
    mut layout: ResMut<ControllerLayoutRes>,
    mut listener_state: ResMut<InputListener>,
) {
    egui::Window::new(CONTROLLER_WINDOW_TITLE).show(egui_ctx.ctx_mut(), |ui| {
        egui::Grid::new(69).show(ui, |ui| {
            for key in PS2_KEY_ORDER {
                // Label with the controller key name
                ui.label(key.to_string());

                // Button with the current binding name/listening prompt
                let listening = listener_state.listening && (key as u8 == listener_state.key_idx);
                if listening {
                    if ui.button(LISTEN_FOR_BINDING.to_string()).clicked() {
                        listener_state.end_listen();
                    }
                } else {
                    let binding_str = &layout
                        .get_binding(key)
                        .map_or(NO_BINDING.to_string(), |key| key.to_string());
                    if ui.button(binding_str).clicked() {
                        listener_state.start_listen(key as u8);
                    };
                };

                ui.end_row();
            }
        });
    });
}

pub fn add_controller_teardown_system(app: &mut App, controller_state: AppState) {}

pub fn add_controller_systems(app: &mut App, controller_state: AppState) {
    // Startup
    app.add_system_set(SystemSet::on_enter(controller_state).with_system(startup));

    // Update
    app.add_system_set(
        SystemSet::on_update(controller_state)
            .with_system(update)
            .with_system(input_listener_system),
    );

    // Teardown
    add_controller_teardown_system(app, controller_state);
}
