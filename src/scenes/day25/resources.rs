use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::Input as InputAsset;

use super::components::{Key, Lock};

#[derive(Debug, Resource)]
pub struct Day25 {
    pub input: Handle<InputAsset>,
    pub camera: Entity,
    pub ui: Entity,
}

#[derive(Debug, Resource)]
pub enum Hovered {
    Lock(Lock),
    Key(Key),
}
