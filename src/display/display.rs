use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use serde::{Deserialize, Serialize};

use super::{analog_stick::AnalogStickParams, button::ButtonParams, frame::FrameParams};

#[derive(Component)]
pub struct RootAtomicDisplayMarker;

pub trait AtomicInputDisplay<P>
where
    P: Clone,
{
    // Spawn an instance of the atomic input display from its parameters.
    // Returns the `Entity` of the root entity associated to the params.
    fn spawn(commands: &mut Commands, params: &P) -> Entity;

    // Add systems to `app` which update all atomic displays of this type.
    fn add_update_systems(app: &mut App);
}

#[derive(Debug, Clone, Serialize, Deserialize, Component, Inspectable)]
pub enum TaggedAtomicParams {
    Button(ButtonParams),
    AnalogStick(AnalogStickParams),
    Frame(FrameParams),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AtomicParamsTag {
    Button,
    AnalogStick,
}

impl AtomicParamsTag {
    pub const CAN_CREATE: [AtomicParamsTag; 2] =
        [AtomicParamsTag::Button, AtomicParamsTag::AnalogStick];
}

impl ToString for AtomicParamsTag {
    fn to_string(&self) -> String {
        match self {
            AtomicParamsTag::Button => "Button".to_string(),
            AtomicParamsTag::AnalogStick => "Analog stick".to_string(),
        }
    }
}

impl Default for AtomicParamsTag {
    fn default() -> Self {
        Self::Button
    }
}

#[derive(Debug, Clone)]
pub struct AtomicDisplay {
    pub params: Box<TaggedAtomicParams>,
    pub entity: Option<Entity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputDisplayMetadata {
    pub title: String,
}

#[derive(Debug)]
pub struct InputDisplay {
    pub metadata: InputDisplayMetadata,
    pub atoms: Vec<AtomicDisplay>,
}

impl Default for InputDisplay {
    fn default() -> Self {
        InputDisplay {
            metadata: InputDisplayMetadata {
                title: "New display".to_string(),
            },
            atoms: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialInputDisplay {
    pub metadata: InputDisplayMetadata,
    pub atoms: Vec<TaggedAtomicParams>,
}

impl Into<InputDisplay> for SerialInputDisplay {
    fn into(self) -> InputDisplay {
        let mut atoms = vec![];
        for params in self.atoms {
            atoms.push(AtomicDisplay {
                params: Box::new(params),
                entity: None,
            });
        }

        InputDisplay {
            atoms,
            metadata: self.metadata,
        }
    }
}
