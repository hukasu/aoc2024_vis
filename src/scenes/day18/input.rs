use bevy::prelude::{Component, Resource};

use crate::{loader::RawInput, tools::Coord};

pub const BOUNDS: Coord = Coord::new(71, 71);

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub bytes: Vec<Coord>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let bytes = input
            .map(|line| {
                let vals = line.split(|c| *c == b',').collect::<Vec<_>>();
                Coord::new(
                    String::from_utf8_lossy(vals[1]).parse().unwrap(),
                    String::from_utf8_lossy(vals[0]).parse().unwrap(),
                )
            })
            .collect();

        Self { bytes }
    }
}
