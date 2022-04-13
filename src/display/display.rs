use bevy_prototype_lyon::prelude::*;

pub enum Displayable {
    RegularPolygon(RegularPolygon),
}

impl Displayable {
    pub fn to_geometry(&self) -> &impl Geometry {
        match self {
            Displayable::RegularPolygon(rp) => rp,
        }
    }
}
