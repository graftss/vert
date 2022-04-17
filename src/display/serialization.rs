use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{Circle, Rectangle},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

// Serialization type for `DrawMode`
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FillModeDef {
    pub options: FillOptions,
    pub color: Color,
}

impl Into<FillMode> for FillModeDef {
    fn into(self) -> FillMode {
        let FillModeDef { options, color } = self;
        FillMode { options, color }
    }
}

impl From<FillMode> for FillModeDef {
    fn from(other: FillMode) -> Self {
        let FillMode { options, color } = other;
        FillModeDef { options, color }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct StrokeModeDef {
    pub options: StrokeOptions,
    pub color: Color,
}

impl Into<StrokeMode> for StrokeModeDef {
    fn into(self) -> StrokeMode {
        let StrokeModeDef { options, color } = self;
        StrokeMode { options, color }
    }
}

impl From<StrokeMode> for StrokeModeDef {
    fn from(other: StrokeMode) -> Self {
        let StrokeMode { options, color } = other;
        StrokeModeDef { options, color }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DrawModeDef {
    /// The shape will be filled using the provided [`FillMode`].
    Fill(FillModeDef),
    /// The shape will be stroked using the provided [`StrokeMode`].
    Stroke(StrokeModeDef),
    /// The shape will be filled with an outline.
    Outlined {
        /// Properties about the filling.
        fill_mode: FillModeDef,
        /// Properties about the outline.
        outline_mode: StrokeModeDef,
    },
}

impl Into<DrawMode> for DrawModeDef {
    fn into(self) -> DrawMode {
        match self {
            Self::Fill(f) => DrawMode::Fill(f.into()),
            Self::Stroke(s) => DrawMode::Stroke(s.into()),
            Self::Outlined {
                fill_mode,
                outline_mode,
            } => DrawMode::Outlined {
                fill_mode: fill_mode.into(),
                outline_mode: outline_mode.into(),
            },
        }
    }
}

impl From<DrawMode> for DrawModeDef {
    fn from(other: DrawMode) -> Self {
        match other {
            DrawMode::Fill(f) => Self::Fill(f.into()),
            DrawMode::Stroke(s) => Self::Stroke(s.into()),
            DrawMode::Outlined {
                fill_mode,
                outline_mode,
            } => Self::Outlined {
                fill_mode: fill_mode.into(),
                outline_mode: outline_mode.into(),
            },
        }
    }
}

// Serialization for `Transform`

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspectable)]
pub struct TransformDef {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Into<Transform> for TransformDef {
    fn into(self) -> Transform {
        let TransformDef {
            translation,
            rotation,
            scale,
        } = self;
        Transform {
            translation,
            rotation,
            scale,
        }
    }
}

impl From<Transform> for TransformDef {
    fn from(other: Transform) -> Self {
        let Transform {
            translation,
            rotation,
            scale,
        } = other;
        TransformDef {
            translation,
            rotation,
            scale,
        }
    }
}

// Serialization for `bevy_prototype_lyon::shapes::Rectangle`.
// Note that this only works for the origin type `RectangleOrigin::BottomLeft`.

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RectangleDef {
    pub extents: Vec2,
}

impl Into<Rectangle> for RectangleDef {
    fn into(self) -> Rectangle {
        let Self { extents } = self;
        Rectangle {
            extents,
            origin: RectangleOrigin::BottomLeft,
        }
    }
}

impl From<Rectangle> for RectangleDef {
    fn from(other: Rectangle) -> Self {
        Self {
            extents: other.extents,
        }
    }
}
