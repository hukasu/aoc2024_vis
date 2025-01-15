use bevy::prelude::Resource;

use super::components::{Key, Lock};

#[derive(Debug, Resource)]
pub enum Hovered {
    Lock(Lock),
    Key(Key),
}
