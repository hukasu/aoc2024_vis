use bevy::prelude::StateSet;

use crate::scenes::{components::PartChange, states::States as SceneStates};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy::prelude::SubStates)]
#[source(SceneStates = SceneStates::Day(24))]
pub enum States {
    #[default]
    Part1,
    Part2,
}

impl From<PartChange> for States {
    fn from(value: PartChange) -> Self {
        match value {
            PartChange::Part1 => super::states::States::Part1,
            PartChange::Part2 => super::states::States::Part2,
        }
    }
}
