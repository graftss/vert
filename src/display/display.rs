use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

use crate::app_state::AppState;

use super::{analog_stick::AnalogStickDisplayData, button::ButtonDisplayData};

#[derive(Clone, Copy)]
pub enum Renderable {
    RegularPolygon(RegularPolygon),
    Circle(Circle),
}

impl Renderable {
    pub fn build_as(&self, mode: DrawMode, transform: bevy::prelude::Transform) -> ShapeBundle {
        match self {
            Renderable::RegularPolygon(rp) => GeometryBuilder::build_as(rp, mode, transform),
            Renderable::Circle(c) => GeometryBuilder::build_as(c, mode, transform),
        }
    }
}

pub enum AtomicDisplay {
    Button(ButtonDisplayData),
    AnalogStick(AnalogStickDisplayData),
}

pub struct Display {
    pub atoms: Vec<AtomicDisplay>,
}
