use bevy::prelude::{Button, Component};

#[derive(Component)]
pub struct Disabled;

#[derive(Debug, Component)]
pub struct StateChange(pub super::states::States);

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub enum PartChange {
    Part1,
    Part2,
}
