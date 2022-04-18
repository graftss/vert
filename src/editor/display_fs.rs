use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{fs, path::Path};

use crate::{
    display::{
        display::{InputDisplay, SerialInputDisplay},
        system::RequestSaveDisplay,
    },
    util::read_from_file,
};

const TOP_PANEL_ID: &'static str = "TOP_PANEL";
const DISPLAYS_DIR_PATH: &'static str = "displays";

pub fn read_saved_displays_dir(mut commands: Commands) {
    let displays_dir = Path::new(DISPLAYS_DIR_PATH);
    if !displays_dir.is_dir() {
        fs::create_dir(DISPLAYS_DIR_PATH);
    }

    let mut displays: Vec<InputDisplay> = vec![];
    if let Ok(readdir) = std::fs::read_dir(DISPLAYS_DIR_PATH) {
        for entry in readdir {
            if let Ok(de) = entry {
                if let Ok(serial_display) =
                    read_from_file::<SerialInputDisplay>(de.path().to_str().unwrap())
                {
                    println!("display: {:?}\n\n\n", serial_display);
                    displays.push(serial_display.into());
                }
            }
        }
    }

    commands.insert_resource(displays);
}

pub fn display_fs_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut ew: EventWriter<RequestSaveDisplay>,
) {
    egui::TopBottomPanel::top(TOP_PANEL_ID).show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal_top(|ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Open", |ui| {
                    if ui.button("Open").clicked() {
                        println!("hi");
                    }
                });

                if ui.button("Save").clicked() {
                    println!("saving");
                    ew.send(RequestSaveDisplay);
                }
            });
        });
    });
}
