use bevy::prelude::*;
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaggedAtomicParams {
    Button(ButtonParams),
    AnalogStick(AnalogStickParams),
    Frame(FrameParams),
}

pub trait AtomicInputDisplay<P>
where
    P: Clone + Copy,
{
    // Spawn an instance of the atomic input display from its parameters.
    fn spawn(commands: &mut Commands, params: &P);

    // Add systems to `app` which update all atomic displays of this type
    // while the app has state `display_state`.
    fn add_update_systems(app: &mut App);

    // Add systems to `app` which teardown all atomic displays of this type
    // when the app leaves the state `display_state`.
    fn add_teardown_systems(app: &mut App);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputDisplayRes {
    pub atoms: Vec<TaggedAtomicParams>,
}

impl Default for InputDisplayRes {
    fn default() -> Self {
        InputDisplayRes { atoms: vec![] }
    }
}

#[derive(Clone)]
pub struct QueuedInputDisplayRes {
    pub display: InputDisplayRes,
}
