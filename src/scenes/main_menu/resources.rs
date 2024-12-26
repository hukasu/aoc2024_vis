use bevy::prelude::{Entity, Resource};

#[derive(Debug, Resource)]
pub struct MainMenu {
    pub camera: Entity,
    pub ui: Entity,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            camera: Entity::PLACEHOLDER,
            ui: Entity::PLACEHOLDER,
        }
    }
}
