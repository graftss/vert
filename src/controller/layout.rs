use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

use crate::input::input::InputSource;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Ps2Key {
    PadU,
    PadL,
    PadD,
    PadR,
    Select,
    Start,
    Square,
    Triangle,
    Circle,
    Cross,
    L1,
    L2,
    L3,
    R1,
    R2,
    R3,
    LeftPosX,
    LeftNegX,
    LeftPosY,
    LeftNegY,
    RightPosX,
    RightNegX,
    RightPosY,
    RightNegY,
}

pub const PS2_KEY_ORDER: [Ps2Key; NUM_PS2_KEYS] = [
    Ps2Key::PadU,
    Ps2Key::PadL,
    Ps2Key::PadD,
    Ps2Key::PadR,
    Ps2Key::Select,
    Ps2Key::Start,
    Ps2Key::Square,
    Ps2Key::Triangle,
    Ps2Key::Circle,
    Ps2Key::Cross,
    Ps2Key::L1,
    Ps2Key::L2,
    Ps2Key::L3,
    Ps2Key::R1,
    Ps2Key::R2,
    Ps2Key::R3,
    Ps2Key::LeftPosX,
    Ps2Key::LeftNegX,
    Ps2Key::LeftPosY,
    Ps2Key::LeftNegY,
    Ps2Key::RightPosX,
    Ps2Key::RightNegX,
    Ps2Key::RightPosY,
    Ps2Key::RightNegY,
];

impl Ps2Key {
    pub fn to_string(self) -> String {
        match self {
            Ps2Key::PadU => "DPad Up".to_string(),
            Ps2Key::PadL => "DPad Left".to_string(),
            Ps2Key::PadD => "DPad Down".to_string(),
            Ps2Key::PadR => "DPad Right".to_string(),
            Ps2Key::Select => "Select".to_string(),
            Ps2Key::Start => "Start".to_string(),
            Ps2Key::Square => "Square".to_string(),
            Ps2Key::Triangle => "Triangle".to_string(),
            Ps2Key::Circle => "Circle".to_string(),
            Ps2Key::Cross => "X".to_string(),
            Ps2Key::L1 => "L1".to_string(),
            Ps2Key::L2 => "L2".to_string(),
            Ps2Key::L3 => "L3".to_string(),
            Ps2Key::R1 => "R1".to_string(),
            Ps2Key::R2 => "R2".to_string(),
            Ps2Key::R3 => "R3".to_string(),
            Ps2Key::LeftPosX => "LS Right".to_string(),
            Ps2Key::LeftNegX => "LS Left".to_string(),
            Ps2Key::LeftPosY => "LS Up".to_string(),
            Ps2Key::LeftNegY => "LS Down".to_string(),
            Ps2Key::RightPosX => "RS Right".to_string(),
            Ps2Key::RightNegX => "RS Left".to_string(),
            Ps2Key::RightPosY => "RS Up".to_string(),
            Ps2Key::RightNegY => "RS Down".to_string(),
        }
    }
}

const NUM_PS2_KEYS: usize = Ps2Key::RightNegY as usize + 1;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ps2Layout {
    pub sources: HashMap<Ps2Key, InputSource>,
}

impl ControllerLayout<Ps2Key> for Ps2Layout {
    fn get_binding(&self, key: Ps2Key) -> Option<&InputSource> {
        self.sources.get(&key)
    }

    fn set_binding(&mut self, key: Ps2Key, source: &InputSource) {
        self.sources.insert(key, *source);
    }

    fn is_source_bound(&self, source: &InputSource) -> Option<ControllerKey> {
        for key in self.sources.keys() {
            if self.sources.get(key) == Some(source) {
                return Some(ControllerKey::Ps2(*key));
            }
        }

        None
    }

    fn get_max_key(&self) -> usize {
        NUM_PS2_KEYS
    }
}

pub trait ControllerLayout<K> {
    fn get_binding(&self, key: K) -> Option<&InputSource>;
    fn set_binding(&mut self, key: K, source: &InputSource);
    fn is_source_bound(&self, source: &InputSource) -> Option<ControllerKey>;
    fn get_max_key(&self) -> usize;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ControllerLayoutsRes {
    pub ps2: Ps2Layout,
}

impl ControllerLayoutsRes {
    pub fn get_binding(&self, key: ControllerKey) -> Option<&InputSource> {
        match key {
            ControllerKey::Ps2(ps2_key) => self.ps2.get_binding(ps2_key),
        }
    }

    pub fn set_binding(&mut self, key: ControllerKey, source: &InputSource) {
        match key {
            ControllerKey::Ps2(ps2_key) => self.ps2.set_binding(ps2_key, source),
        }
    }

    pub fn is_source_bound(&self, source: &InputSource) -> Option<ControllerKey> {
        self.ps2.is_source_bound(source)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControllerKey {
    Ps2(Ps2Key),
}

impl ControllerKey {
    pub fn to_string(&self) -> String {
        match self {
            ControllerKey::Ps2(ps2_key) => ps2_key.to_string(),
        }
    }
}
