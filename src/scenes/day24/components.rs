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

#[derive(Debug, Component)]
pub struct Adder;

#[derive(Debug, Component)]
pub struct Gate {
    pub left: [u8; 3],
    pub right: [u8; 3],
    pub out: [u8; 3],
}

#[derive(Debug, Component)]
pub struct GizmosCamera;
