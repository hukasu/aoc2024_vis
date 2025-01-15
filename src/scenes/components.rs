use bevy::prelude::{Button, Component};

use super::states::Scene;

#[derive(Component)]
pub struct Disabled;

#[derive(Debug, Component)]
pub struct SceneChange(pub Scene);

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub enum PartChange {
    Part1,
    Part2,
}
