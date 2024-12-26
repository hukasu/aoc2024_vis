#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy::prelude::States)]
pub enum States {
    #[default]
    MainMenu,
    Day(u8),
}
