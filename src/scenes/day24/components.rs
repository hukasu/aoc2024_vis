use bevy::prelude::{Button, Component, Event};

#[derive(Debug, Event)]
#[require(Button)]
pub enum Controls {
    Reset,
    Step,
    FastForward,
}
