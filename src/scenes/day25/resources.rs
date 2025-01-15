use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::RawInput;

use super::components::{Key, Lock};

#[derive(Debug, Resource)]
pub struct Day25 {
    pub input: Handle<RawInput>,
    pub camera: Entity,
    pub ui: Entity,
}

#[derive(Debug, Resource)]
pub enum Hovered {
    Lock(Lock),
    Key(Key),
}
