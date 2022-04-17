use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::display::display::{InputDisplay, TaggedAtomicParams};

pub trait Inspectable {}

pub fn display_inspector_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut display_res: Option<ResMut<InputDisplay>>,
) {
    if let None = display_res {
        return;
    }
    let mut display = display_res.unwrap();

    egui::Window::new("Editor").show(egui_ctx.ctx_mut(), |ui| {
        for atom in display.atoms.iter_mut() {
            ui.label("hi");
        }
    });
}
