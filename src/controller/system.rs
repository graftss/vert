use bevy::{prelude::*, utils::HashMap};
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;

use crate::{
    input::{input::InputSource, raw_input_reader::RawInputRes, RawInputReader},
    state::AppState,
    util::{despawn_all_with, read_from_file, write_to_file},
};

use super::{
    layout::{
        ControllerKey, ControllerLayout, ControllerLayoutsRes, Ps2Key, Ps2Layout, PS2_KEY_ORDER,
    },
    listener::{cleanup_input_listener_system, input_listener_system, InputListener},
};

pub const LAYOUTS_FILE_PATH: &'static str = "layouts.json";

pub fn startup(mut commands: Commands) {
    // Read and insert the layouts file from disk.
    match read_from_file::<ControllerLayoutsRes>(LAYOUTS_FILE_PATH) {
        Ok(layouts) => {
            commands.insert_resource(layouts);
        }
        Err(e) => {
            println!("Error in `read_layouts_file`: {:?}", e);
            commands.insert_resource(ControllerLayoutsRes::default());
        }
    }

    // Insert the `InputListener` resources.
    commands.insert_resource(InputListener::default());
}

const NO_BINDING: &'static str = "-";
const LISTEN_FOR_BINDING: &'static str = "Listening...";
const CONTROLLER_WINDOW_TITLE: &'static str = "Controller";

pub fn ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut layout: ResMut<ControllerLayoutsRes>,
    mut input_listener: ResMut<InputListener>,
) {
    egui::Window::new(CONTROLLER_WINDOW_TITLE).show(egui_ctx.ctx_mut(), |ui| {
        egui::Grid::new(69).show(ui, |ui| {
            for ps2_key in PS2_KEY_ORDER {
                let key = ControllerKey::Ps2(ps2_key);
                // Label with the controller key name
                ui.label(key.to_string());

                // Button with the current binding name/listening prompt
                let listening = input_listener.is_listening()
                    && input_listener.key == Some(ControllerKey::Ps2(ps2_key));

                if listening {
                    if ui.button(LISTEN_FOR_BINDING.to_string()).clicked() {
                        input_listener.end_listen();
                    }
                } else {
                    let binding_str = &layout
                        .get_binding(key)
                        .map_or(NO_BINDING.to_string(), |key| key.to_string());

                    if ui.button(binding_str).clicked() {
                        input_listener.start_listen(key);
                    };
                };

                ui.end_row();
            }
        });
    });
}

fn save_layouts(layouts: Res<ControllerLayoutsRes>) {
    write_to_file(&layouts.into_inner(), LAYOUTS_FILE_PATH);
}

pub fn add_controller_teardown_system(app: &mut App, controller_state: AppState) {
    app.add_system_set(SystemSet::on_exit(controller_state).with_system(save_layouts));
}

pub fn add_controller_systems(app: &mut App, controller_state: AppState) {
    // Startup
    app.add_startup_system(startup);

    // Update
    app.add_system_set(
        SystemSet::on_update(controller_state)
            .with_system(ui_system)
            .with_system(input_listener_system)
            .with_system(cleanup_input_listener_system),
    );

    // Teardown
    add_controller_teardown_system(app, controller_state);
}
