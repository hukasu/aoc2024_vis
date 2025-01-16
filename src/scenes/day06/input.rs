use std::collections::BTreeSet;

use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub lines: Vec<Vec<u8>>,
    pub paths: BTreeSet<(usize, usize)>,
    pub possible_obstacles: BTreeSet<(usize, usize)>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let starting_position = input
            .iter()
            .enumerate()
            .find_map(|(j, line)| line.iter().position(|c| *c == b'^').map(|i| (i, j)))
            .unwrap();
        let direction = (0isize, -1isize);

        let paths = Self::get_path(input.as_slice(), starting_position, direction, None).unwrap();

        let possible_obstacles = paths
            .iter()
            .copied()
            .filter(|obstacle| obstacle != &starting_position)
            .filter(|obstacle| {
                Self::get_path(
                    input.as_slice(),
                    starting_position,
                    direction,
                    Some(*obstacle),
                )
                .is_none()
            })
            .collect();

        Self {
            lines: input.into_iter().map(Vec::from).collect(),
            paths,
            possible_obstacles,
        }
    }

    fn get_path(
        input: &[&[u8]],
        starting_position: (usize, usize),
        direction: (isize, isize),
        extra_obstacle: Option<(usize, usize)>,
    ) -> Option<BTreeSet<(usize, usize)>> {
        let mut paths = BTreeSet::new();

        let mut position = starting_position;
        let mut direction = direction;

        loop {
            let (Some(x), Some(y)) = (
                position.0.checked_add_signed(direction.0),
                position.1.checked_add_signed(direction.1),
            ) else {
                break;
            };

            if x >= input[0].len() || y >= input.len() {
                break;
            }

            if input[y][x] == b'#'
                || extra_obstacle
                    .filter(|obstacle| obstacle == &(x, y))
                    .is_some()
            {
                direction = Self::next_direction(direction);
            } else {
                position = (x, y);
            }

            if !paths.insert((position, direction)) {
                return None;
            }
        }

        Some(paths.into_iter().map(|(pos, _)| pos).collect())
    }

    fn next_direction(direction: (isize, isize)) -> (isize, isize) {
        match direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => unreachable!("Invalid direction."),
        }
    }
}
