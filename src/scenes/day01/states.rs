use bevy::prelude::{ComputedStates, StateSet, SubStates};

use crate::scenes::{components::PartChange, states::States as SceneStates};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy::prelude::SubStates)]
#[source(SceneStates = SceneStates::Day(1))]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(SceneStates = SceneStates::Day(1))]
pub enum InputState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(SceneStates = SceneStates::Day(1))]
pub enum UiState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualizationState {
    WaitingInput,
    WaitingUi,
    Ready,
}

impl ComputedStates for VisualizationState {
    type SourceStates = (SceneStates, InputState, UiState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            (SceneStates::Day(1), InputState::NotLoaded, _) => Some(Self::WaitingInput),
            (SceneStates::Day(1), InputState::Loaded, UiState::NotLoaded) => Some(Self::WaitingUi),
            (SceneStates::Day(1), InputState::Loaded, UiState::Loaded) => Some(Self::Ready),
            _ => None,
        }
    }
}
