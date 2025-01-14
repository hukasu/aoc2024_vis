use bevy::prelude::{ComputedStates, StateSet, SubStates};

use crate::scenes::states::States as SceneStates;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(SceneStates = SceneStates::Day(2))]
pub enum InputState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(SceneStates = SceneStates::Day(2))]
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
            (SceneStates::Day(2), InputState::NotLoaded, _) => Some(Self::WaitingInput),
            (SceneStates::Day(2), InputState::Loaded, UiState::NotLoaded) => Some(Self::WaitingUi),
            (SceneStates::Day(2), InputState::Loaded, UiState::Loaded) => Some(Self::Ready),
            _ => None,
        }
    }
}
