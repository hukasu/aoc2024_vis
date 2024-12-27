use std::collections::BTreeMap;

use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::Input as InputAsset;

use super::operation::Operation;

#[derive(Debug, Resource)]
pub struct Day24 {
    pub input: Handle<InputAsset>,
    pub camera: Entity,
    pub ui: Entity,
}

#[derive(Debug, Resource)]
pub struct Input {
    pub x: [u8; 64],
    pub y: [u8; 64],
    pub z: [u8; 64],
    pub intermediate: BTreeMap<[u8; 3], u8>,
    pub operations: Vec<Operation>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            x: [0; 64],
            y: [0; 64],
            z: [0; 64],
            intermediate: BTreeMap::new(),
            operations: Vec::new(),
        }
    }
}
