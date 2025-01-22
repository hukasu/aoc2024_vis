use bevy::prelude::{Button, Component, Deref};

#[derive(Debug, Component, Deref)]
#[require(Button)]
pub struct Start(pub (usize, usize));

#[derive(Debug, Clone, Component)]
pub struct PartOfTrail {
    pub coord: (usize, usize),
    pub starts: Vec<(usize, usize)>,
    pub is_end: bool,
}
