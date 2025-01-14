use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::Input as InputAsset;

#[derive(Debug, Resource)]
pub struct Day03 {
    pub input: Handle<InputAsset>,
    pub camera: Entity,
    pub ui: Entity,
}
