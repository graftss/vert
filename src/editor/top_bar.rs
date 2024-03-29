use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{fs, path::Path};

use crate::display::{
    display::AtomicParamsTag,
    system::{RequestLoadDisplay, RequestSaveDisplay, RequestSpawnAtom},
};

const TOP_PANEL_ID: &'static str = "TOP_PANEL";
const DISPLAYS_DIR_PATH: &'static str = "displays";

#[derive(Debug)]
pub struct SavedInputDisplays {
    paths: Vec<String>,
}

pub fn read_saved_displays_dir() -> SavedInputDisplays {
    let displays_dir = Path::new(DISPLAYS_DIR_PATH);
    if !displays_dir.is_dir() {
        fs::create_dir(DISPLAYS_DIR_PATH);
    }

    let mut paths: Vec<String> = vec![];
    if let Ok(readdir) = std::fs::read_dir(DISPLAYS_DIR_PATH) {
        for entry in readdir {
            if let Ok(de) = entry {
                paths.push(de.path().to_str().unwrap().to_string());
            }
        }
    }

    SavedInputDisplays { paths }
}

#[derive(Debug, Default)]
pub struct TopBarState {
    pub display_name: String,
    pub created_atom: AtomicParamsTag,
}

pub fn top_bar_startup_system(mut commands: Commands) {
    commands.insert_resource(TopBarState::default());
}

pub fn display_top_bar_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<TopBarState>,
    mut ew_save: EventWriter<RequestSaveDisplay>,
    mut ew_load: EventWriter<RequestLoadDisplay>,
    mut ew_spawn: EventWriter<RequestSpawnAtom>,
) {
    egui::TopBottomPanel::top(TOP_PANEL_ID).show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal_top(|ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Open Display", |ui| {
                    // stop the "Open Display" dropdown menu from being empty (if there aren't any saved files)
                    let mut empty_menu = true;
                    let saved_displays = read_saved_displays_dir();
                    for path in saved_displays.paths.iter() {
                        empty_menu = false;
                        if ui.button(path).clicked() {
                            ew_load.send(RequestLoadDisplay(path.clone()));
                            ui.close_menu();
                        }
                    }

                    if empty_menu {
                        ui.label("No saved displays.");
                    }
                });

                ui.separator();

                // button to "Save" the current input display
                if ui.button("Save as:").clicked() {
                    ew_save.send(RequestSaveDisplay);
                }

                ui.add_sized(
                    bevy_egui::egui::Vec2::new(90.0, 20.0),
                    egui::TextEdit::singleline(&mut state.display_name),
                );

                ui.separator();

                if ui.button("Add: ").clicked() {
                    ew_spawn.send(RequestSpawnAtom::New(state.created_atom, Vec2::ZERO));
                }

                egui::ComboBox::new("create_atom", "")
                    .selected_text(state.created_atom.to_string())
                    .show_ui(ui, |ui| {
                        for atom in AtomicParamsTag::CAN_CREATE {
                            ui.selectable_value(&mut state.created_atom, atom, atom.to_string());
                        }
                    });
            });
        });
    });
}
