use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::display::{
    display::{InputDisplay, TaggedAtomicParams},
    system::RequestDespawnAtom,
};

use super::system::TestState;

pub trait Inspectable {}

pub fn display_inspector_system(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut display_res: Option<ResMut<InputDisplay>>,
    mut event_writer: EventWriter<RequestDespawnAtom>,
    mut test: ResMut<TestState>,
) {
    if let Some(mut display) = display_res {
        egui::Window::new("Editor").show(egui_ctx.ctx_mut(), |ui| {});
    }
}
