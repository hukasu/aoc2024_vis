use bevy::prelude::Component;

#[derive(Component)]
pub struct Disabled;

#[derive(Debug, Component)]
pub struct StateChange(pub super::states::States);
