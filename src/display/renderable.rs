use super::serialization::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Inspectable)]
pub enum Renderable {
    RegularPolygon(RegularPolygonDef),
    Circle(CircleDef),
    Rectangle(RectangleDef),
}

impl Renderable {
    pub fn build_as(&self, mode: DrawMode, transform: bevy::prelude::Transform) -> ShapeBundle {
        use bevy_prototype_lyon::shapes::*;

        match *self {
            Renderable::RegularPolygon(rp) => {
                let trp: RegularPolygon = rp.into();
                GeometryBuilder::build_as(&trp, mode, transform)
            }
            Renderable::Circle(c) => {
                let tc: Circle = c.into();
                GeometryBuilder::build_as(&tc, mode, transform)
            }
            Renderable::Rectangle(r) => {
                let r: Rectangle = r.into();
                GeometryBuilder::build_as(&r, mode, transform)
            }
        }
    }
}
