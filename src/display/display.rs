use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

use super::{
    analog_stick::AnalogStickParams,
    button::ButtonParams,
    serialization::{CircleDef, RegularPolygonDef},
};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Renderable {
    RegularPolygon(RegularPolygonDef),
    Circle(CircleDef),
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
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TaggedAtomicParams {
    Button(ButtonParams),
    AnalogStick(AnalogStickParams),
}

pub trait AtomicInputDisplay<P>
where
    P: Clone + Copy,
{
    // Spawn an instance of the atomic input display from its parameters.
    fn spawn(commands: &mut Commands, params: &P);

    // Add systems to `app` which update all atomic displays of this type
    // while the app has state `display_state`.
    fn add_update_systems(app: &mut App, display_state: AppState);

    // Add systems to `app` which teardown all atomic displays of this type
    // when the app leaves the state `display_state`.
    fn add_teardown_systems(app: &mut App, display_state: AppState);
}

#[derive(Serialize, Deserialize)]
pub struct InputDisplayRes {
    pub atoms: Vec<TaggedAtomicParams>,
}

impl Default for InputDisplayRes {
    fn default() -> Self {
        InputDisplayRes { atoms: vec![] }
    }
}
