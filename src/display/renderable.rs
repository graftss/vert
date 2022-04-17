use super::serialization::*;
use bevy::{ecs::system::EntityCommands, prelude::Bundle};
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Inspectable)]
pub enum Renderable {
    None,
    RegularPolygon(RegularPolygonDef),
    Circle(CircleDef),
    Rectangle(RectangleDef),
}

impl Renderable {
    pub fn insert_bundle(
        &self,
        mut commands: &mut EntityCommands,
        mode: DrawMode,
        transform: bevy::prelude::Transform,
    ) {
        use bevy_prototype_lyon::shapes::*;

        match *self {
            Renderable::None => {}
            Renderable::RegularPolygon(rp) => {
                let trp: RegularPolygon = rp.into();
                commands.insert_bundle(GeometryBuilder::build_as(&trp, mode, transform));
            }
            Renderable::Circle(c) => {
                let tc: Circle = c.into();
                commands.insert_bundle(GeometryBuilder::build_as(&tc, mode, transform));
            }
            Renderable::Rectangle(r) => {
                let r: Rectangle = r.into();
                commands.insert_bundle(GeometryBuilder::build_as(&r, mode, transform));
            }
        }
    }
}
