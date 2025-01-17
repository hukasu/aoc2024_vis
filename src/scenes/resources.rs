use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
    text::Font,
};

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct GenericDay {
    pub input: Handle<RawInput>,
    pub camera: Entity,
    pub ui: Entity,
}

#[derive(Debug, Resource)]
pub struct FontHandles {
    pub font: Handle<Font>,
    pub symbol1: Handle<Font>,
    pub symbol2: Handle<Font>,
}
