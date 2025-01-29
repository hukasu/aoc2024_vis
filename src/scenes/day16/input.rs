use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub input: Vec<u8>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        Self {
            input: input.0.clone(),
        }
    }
}
