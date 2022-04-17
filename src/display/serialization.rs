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
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspectable)]
pub struct FillModeDef {
    #[inspectable(ignore)]
    pub options: FillOptions,
    pub color: Color,
}

impl Default for FillModeDef {
    fn default() -> Self {
        Self {
            options: Default::default(),
            color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspectable)]
pub struct StrokeOptionsDef {
    #[inspectable(min = 1.0, max = 20.0, suffix = "px", label = "Thickness")]
    pub line_width: f32,
}

impl Into<StrokeOptions> for StrokeOptionsDef {
    fn into(self) -> StrokeOptions {
        let StrokeOptionsDef { line_width } = self;
        StrokeOptions::default().with_line_width(line_width)
    }
}

impl From<StrokeOptions> for StrokeOptionsDef {
    fn from(other: StrokeOptions) -> Self {
        let StrokeOptions { line_width, .. } = other;
        StrokeOptionsDef { line_width }
    }
}

impl Default for StrokeOptionsDef {
    fn default() -> Self {
        Self { line_width: 1.0 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspectable)]
pub struct StrokeModeDef {
    #[inspectable(label = "Options")]
    pub options: StrokeOptionsDef,
    #[inspectable(label = "Color")]
    pub color: Color,
}

impl Into<StrokeMode> for StrokeModeDef {
    fn into(self) -> StrokeMode {
        let StrokeModeDef { options, color } = self;
        StrokeMode {
            options: options.into(),
            color,
        }
    }
}

impl From<StrokeMode> for StrokeModeDef {
    fn from(other: StrokeMode) -> Self {
        let StrokeMode { options, color } = other;
        Self {
            options: options.into(),
            color,
        }
    }
}

impl Default for StrokeModeDef {
    fn default() -> Self {
        Self {
            options: Default::default(),
            color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Inspectable)]
pub enum DrawModeDef {
    Fill(FillModeDef),
    Stroke(StrokeModeDef),
    Outlined {
        #[inspectable(label = "Fill")]
        fill_mode: FillModeDef,
        #[inspectable(label = "Outline")]
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
