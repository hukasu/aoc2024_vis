use std::collections::BTreeSet;

use bevy::prelude::Resource;

use crate::{loader::RawInput, tools::Direction};

type Coord = (usize, usize);

#[derive(Debug, Resource)]
pub struct Input {
    pub walls: BTreeSet<Coord>,
    pub boxes: BTreeSet<Coord>,
    pub robot: Coord,
    pub instructions: Vec<Direction>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let mut input = input.split(|c| *c == b'\n').enumerate();

        let mut walls = BTreeSet::new();
        let mut boxes = BTreeSet::new();
        let mut robot = (0, 0);

        loop {
            let (y, line) = input.next().unwrap();
            if line.is_empty() {
                break;
            }

            for (x, tile) in line.iter().enumerate() {
                match tile {
                    b'#' => {
                        walls.insert((x, y));
                    }
                    b'O' => {
                        boxes.insert((x, y));
                    }
                    b'@' => robot = (x, y),
                    b'.' => (),
                    _ => unreachable!("Invalid tile"),
                }
            }
        }

        let instructions = input
            .flat_map(|(_, line)| {
                line.iter().map(|mov| match mov {
                    b'^' => Direction::North,
                    b'v' => Direction::South,
                    b'<' => Direction::West,
                    b'>' => Direction::East,
                    _ => unreachable!("Invalid move."),
                })
            })
            .collect::<Vec<_>>();

        Self {
            walls,
            boxes,
            robot,
            instructions,
        }
    }
}
