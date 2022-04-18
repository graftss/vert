use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{fs, path::Path};

use crate::{
    display::{
        display::{InputDisplay, SerialInputDisplay},
        system::{RequestLoadDisplay, RequestSaveDisplay},
    },
    util::read_from_file,
};

const TOP_PANEL_ID: &'static str = "TOP_PANEL";
const DISPLAYS_DIR_PATH: &'static str = "displays";

#[derive(Debug)]
pub struct SavedInputDisplays {
    paths: Vec<String>,
}

pub fn read_saved_displays_dir(mut commands: Commands) {
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

    commands.insert_resource(SavedInputDisplays { paths });
}

pub fn display_fs_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut ew_save: EventWriter<RequestSaveDisplay>,
    mut ew_load: EventWriter<RequestLoadDisplay>,
    saved_displays_res: Option<Res<SavedInputDisplays>>,
) {
    egui::TopBottomPanel::top(TOP_PANEL_ID).show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal_top(|ui| {
            egui::menu::bar(ui, |ui| {
                // menu to "Open" a saved input display
                ui.menu_button("Open", |ui| {
                    if let Some(saved_displays) = saved_displays_res {
                        for path in saved_displays.paths.iter() {
                            if ui.button(path).clicked() {
                                ew_load.send(RequestLoadDisplay(path.clone()));
                                ui.close_menu();
                            }
                        }
                    }
                });

                // button to "Save" the current input display
                if ui.button("Save").clicked() {
                    println!("saving");
                    ew_save.send(RequestSaveDisplay);
                }
            });
        });
    });
}
