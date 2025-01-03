use bevy::prelude::{Button, Component, Deref};

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub enum Controls {
    Reset,
    Step,
    FastForward,
}

#[derive(Debug, Clone, Copy, Component, Deref)]
pub struct Wire(pub [u8; 3]);
