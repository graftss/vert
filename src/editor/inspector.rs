use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::display::{
    display::{InputDisplay, TaggedAtomicParams},
    system::RequestDespawnAtom,
};

pub trait Inspectable {}

pub fn display_inspector_system(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut display_res: Option<ResMut<InputDisplay>>,
    mut event_writer: EventWriter<RequestDespawnAtom>,
) {
    if let None = display_res {
        return;
    }
    let mut display = display_res.unwrap();

    egui::Window::new("Editor").show(egui_ctx.ctx_mut(), |ui| {
        for (i, mut atom) in display.atoms.iter_mut().enumerate() {
            ui.label(format!("hi {:?}", atom.entity));

            // delete atom button
            if ui.button("delete").clicked() {
                event_writer.send(RequestDespawnAtom(i))
            }
        }
    });
}
