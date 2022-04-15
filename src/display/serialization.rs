use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use serde::{Deserialize, Serialize};

// Serialization type for `RegularPolygon`

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegularPolygonFeatureDef {
    /// The radius of the polygon's circumcircle.
    Radius(f32),
    /// The radius of the polygon's incircle.
    Apothem(f32),
    /// The length of the polygon's side.
    SideLength(f32),
}

impl Into<RegularPolygonFeature> for RegularPolygonFeatureDef {
    fn into(self) -> RegularPolygonFeature {
        match self {
            Self::Radius(v) => RegularPolygonFeature::Radius(v),
            Self::Apothem(v) => RegularPolygonFeature::Apothem(v),
            Self::SideLength(v) => RegularPolygonFeature::SideLength(v),
        }
    }
}

impl From<RegularPolygonFeature> for RegularPolygonFeatureDef {
    fn from(other: RegularPolygonFeature) -> Self {
        match other {
            RegularPolygonFeature::Radius(v) => Self::Radius(v),
            RegularPolygonFeature::Apothem(v) => Self::Apothem(v),
            RegularPolygonFeature::SideLength(v) => Self::SideLength(v),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegularPolygonDef {
    pub sides: usize,
    pub center: Vec2,
    pub feature: RegularPolygonFeatureDef,
}

impl Into<RegularPolygon> for RegularPolygonDef {
    fn into(self) -> RegularPolygon {
        RegularPolygon {
            sides: self.sides,
            center: self.center,
            feature: self.feature.into(),
        }
    }
}

impl From<RegularPolygon> for RegularPolygonDef {
    fn from(other: RegularPolygon) -> Self {
        RegularPolygonDef {
            sides: other.sides,
            center: other.center,
            feature: other.feature.into(),
        }
    }
}

impl Default for RegularPolygonDef {
    fn default() -> Self {
        Self {
            sides: 3,
            center: Vec2::ZERO,
            feature: RegularPolygonFeatureDef::Radius(1.0),
        }
    }
}

// Serialization type for `Circle`

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CircleDef {
    pub radius: f32,
    pub center: Vec2,
}

impl Into<Circle> for CircleDef {
    fn into(self) -> Circle {
        Circle {
            radius: self.radius,
            center: self.center,
        }
    }
}

impl From<Circle> for CircleDef {
    fn from(other: Circle) -> Self {
        CircleDef {
            radius: other.radius,
            center: other.center,
        }
    }
}

impl Default for CircleDef {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec2::ZERO,
        }
    }
}
