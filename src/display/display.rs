use bevy::prelude::*;
use bevy_inspector_egui::{egui::Ui, Context, Inspectable};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::*,
    shapes::{Circle, Rectangle},
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

use super::{
    analog_stick::AnalogStickParams,
    button::ButtonParams,
    frame::FrameParams,
    serialization::{CircleDef, RectangleDef, RegularPolygonDef},
};

#[derive(Component)]
pub struct RootAtomicDisplayMarker;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Renderable {
    RegularPolygon(RegularPolygonDef),
    Circle(CircleDef),
    Rectangle(RectangleDef),
}

impl Renderable {
    pub fn build_as(&self, mode: DrawMode, transform: bevy::prelude::Transform) -> ShapeBundle {
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

pub trait AtomicInputDisplay<P>
where
    P: Clone + Copy,
{
    // Spawn an instance of the atomic input display from its parameters.
    // Returns the `Entity` of the root entity associated to the params.
    fn spawn(commands: &mut Commands, params: &P) -> Entity;

    // Add systems to `app` which update all atomic displays of this type
    // while the app has state `display_state`.
    fn add_update_systems(app: &mut App);

    // Add systems to `app` which teardown all atomic displays of this type
    // when the app leaves the state `display_state`.
    fn add_teardown_systems(app: &mut App);
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaggedAtomicParams {
    Button(ButtonParams),
    AnalogStick(AnalogStickParams),
    Frame(FrameParams),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AtomicDisplay {
    pub params: TaggedAtomicParams,

    // If `Some(entity)`, the root entity associated with this atomic display.
    #[serde(skip_serializing, default)]
    pub entity: Option<Entity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDisplay {
    pub atoms: Vec<AtomicDisplay>,
}

impl Default for InputDisplay {
    fn default() -> Self {
        InputDisplay { atoms: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedInputDisplay {
    pub display: InputDisplay,
}
