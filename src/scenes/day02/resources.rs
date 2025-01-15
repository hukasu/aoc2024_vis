use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Day02 {
    pub input: Handle<RawInput>,
    pub camera: Entity,
    pub ui: Entity,
}
