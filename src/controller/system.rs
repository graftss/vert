use bevy::{prelude::*, utils::HashMap};
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;

use crate::{
    input::{
        input::InputSource,
        listener::{
            cleanup_input_listener_system, input_listener_system, InputListener, ListenerResult,
        },
        raw_input_reader::RawInputRes,
        RawInputReader,
    },
    state::AppState,
    util::{despawn_all_with, read_from_file, write_to_file},
};

use super::layout::{
    ControllerKey, ControllerLayout, ControllerLayoutsRes, Ps2Key, Ps2Layout, PS2_KEY_ORDER,
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
    mut layouts: ResMut<ControllerLayoutsRes>,
    mut input_listener: ResMut<InputListener>,
    mut event_reader: EventReader<ListenerResult>,
) {
    egui::Window::new(CONTROLLER_WINDOW_TITLE).show(egui_ctx.ctx_mut(), |ui| {
        egui::Grid::new(69).show(ui, |ui| {
            for ps2_key in PS2_KEY_ORDER {
                let key = ControllerKey::Ps2(ps2_key);
                // Label with the controller key name
                ui.label(key.to_string());

                // Button with the current binding name/listening prompt
                let listening = input_listener.listening_for_input_source()
                    && input_listener.has_key_consumer(ControllerKey::Ps2(ps2_key));

                if listening {
                    ui.button(LISTEN_FOR_BINDING.to_string());
                    for ev in event_reader.iter() {
                        if let ListenerResult::SourceToKey(source, key) = ev {
                            layouts.set_binding(*key, source);
                            write_layouts_to_file(&layouts);
                            input_listener.stop_listening();
                        }
                    }
                } else {
                    let binding_str = &layouts
                        .get_binding(key)
                        .map_or(NO_BINDING.to_string(), |key| key.to_string());

                    if ui.button(binding_str).clicked() {
                        input_listener.listen_input_source(key);
                    };
                };

                ui.end_row();
            }
        });
    });
}

fn write_layouts_to_file(layouts: &ControllerLayoutsRes) {
    write_to_file(layouts, LAYOUTS_FILE_PATH);
}

pub fn add_controller_systems(app: &mut App, controller_state: AppState) {
    // Startup
    app.add_startup_system(startup);

    // Update
    app.add_system_set(SystemSet::on_update(controller_state).with_system(ui_system));
}
