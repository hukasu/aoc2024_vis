use bevy::prelude::{Deref, Resource};

#[derive(Debug, Resource, Deref)]
pub struct HoveredTile(pub (usize, usize));
