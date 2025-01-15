use bevy::prelude::{ComputedStates, StateSet, States, SubStates};

use super::components::PartChange;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Scene {
    #[default]
    MainMenu,
    Day(u8),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(Scene = Scene::Day(_))]
pub enum Part {
    #[default]
    Part1,
    Part2,
}

impl From<PartChange> for Part {
    fn from(value: PartChange) -> Self {
        match value {
            PartChange::Part1 => Part::Part1,
            PartChange::Part2 => Part::Part2,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(Scene = Scene::Day(_))]
pub enum InputState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(Scene = Scene::Day(_))]
pub enum UiState {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualizationState<const N: u8> {
    WaitingInput,
    WaitingUi,
    Ready,
}

impl<const N: u8> ComputedStates for VisualizationState<N> {
    type SourceStates = (Scene, InputState, UiState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            (Scene::Day(day), InputState::NotLoaded, _) if day == N => Some(Self::WaitingInput),
            (Scene::Day(day), InputState::Loaded, UiState::NotLoaded) if day == N => {
                Some(Self::WaitingUi)
            }
            (Scene::Day(day), InputState::Loaded, UiState::Loaded) if day == N => Some(Self::Ready),
            _ => None,
        }
    }
}
