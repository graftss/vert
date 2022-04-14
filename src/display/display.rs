use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

pub enum Displayable {
    RegularPolygon(RegularPolygon),
    Circle(Circle),
}

impl Displayable {
    pub fn build_as(&self, mode: DrawMode, transform: bevy::prelude::Transform) -> ShapeBundle {
        match self {
            Displayable::RegularPolygon(rp) => GeometryBuilder::build_as(rp, mode, transform),
            Displayable::Circle(c) => GeometryBuilder::build_as(c, mode, transform),
        }
    }
}
