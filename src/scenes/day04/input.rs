use std::collections::BTreeSet;

use bevy::prelude::Resource;

use crate::loader::RawInput;

const XMAS: &[u8] = b"MAS";
const MAS: &[u8] = b"AS";

#[derive(Debug, Resource)]
pub struct Input {
    pub lines: Vec<Vec<u8>>,
    pub positions_xmas: BTreeSet<(usize, usize)>,
    pub positions_x_mas: BTreeSet<(usize, usize)>,
    pub result_part1: u32,
    pub result_part2: u32,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').collect::<Vec<_>>();

        let mut positions_xmas = BTreeSet::new();
        let mut positions_x_mas = BTreeSet::new();

        let mut result_part1 = 0;
        let mut result_part2 = 0;

        for (i, line) in input.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == b'X' {
                    if Self::test_left(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([(j, i), (j + 1, i), (j + 2, i), (j + 3, i)]);
                        result_part1 += 1;
                    }
                    if Self::test_right(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([(j, i), (j - 1, i), (j - 2, i), (j - 3, i)]);
                        result_part1 += 1;
                    }
                    if Self::test_up(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([(j, i), (j, i - 1), (j, i - 2), (j, i - 3)]);
                        result_part1 += 1;
                    }
                    if Self::test_down(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([(j, i), (j, i + 1), (j, i + 2), (j, i + 3)]);
                        result_part1 += 1;
                    }
                    if Self::test_ne(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([
                            (j, i),
                            (j + 1, i - 1),
                            (j + 2, i - 2),
                            (j + 3, i - 3),
                        ]);
                        result_part1 += 1;
                    }
                    if Self::test_se(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([
                            (j, i),
                            (j + 1, i + 1),
                            (j + 2, i + 2),
                            (j + 3, i + 3),
                        ]);
                        result_part1 += 1;
                    }
                    if Self::test_nw(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([
                            (j, i),
                            (j - 1, i - 1),
                            (j - 2, i - 2),
                            (j - 3, i - 3),
                        ]);
                        result_part1 += 1;
                    }
                    if Self::test_sw(input.as_slice(), (j, i), XMAS) {
                        positions_xmas.extend([
                            (j, i),
                            (j - 1, i + 1),
                            (j - 2, i + 2),
                            (j - 3, i + 3),
                        ]);
                        result_part1 += 1;
                    }
                } else if *c == b'M' {
                    if line.get(j + 2).filter(|c| **c == b'M').is_some() {
                        if Self::test_ne(input.as_slice(), (j, i), MAS)
                            && Self::test_nw(input.as_slice(), (j + 2, i), MAS)
                        {
                            positions_x_mas.extend([
                                (j, i),
                                (j + 1, i - 1),
                                (j + 2, i - 2),
                                (j + 2, i),
                                (j, i - 2),
                            ]);
                            result_part2 += 1;
                        }
                        if Self::test_se(input.as_slice(), (j, i), MAS)
                            && Self::test_sw(input.as_slice(), (j + 2, i), MAS)
                        {
                            positions_x_mas.extend([
                                (j, i),
                                (j + 1, i + 1),
                                (j + 2, i + 2),
                                (j + 2, i),
                                (j, i + 2),
                            ]);
                            result_part2 += 1;
                        }
                    }
                    if input
                        .get(i + 2)
                        .and_then(|line| line.get(j))
                        .filter(|c| **c == b'M')
                        .is_some()
                    {
                        if Self::test_se(input.as_slice(), (j, i), MAS)
                            && Self::test_ne(input.as_slice(), (j, i + 2), MAS)
                        {
                            positions_x_mas.extend([
                                (j, i),
                                (j + 1, i + 1),
                                (j + 2, i + 2),
                                (j, i + 2),
                                (j + 2, i),
                            ]);
                            result_part2 += 1;
                        }
                        if Self::test_sw(input.as_slice(), (j, i), MAS)
                            && Self::test_nw(input.as_slice(), (j, i + 2), MAS)
                        {
                            positions_x_mas.extend([
                                (j, i),
                                (j - 1, i + 1),
                                (j - 2, i + 2),
                                (j, i + 2),
                                (j - 2, i),
                            ]);
                            result_part2 += 1;
                        }
                    }
                }
            }
        }

        Self {
            lines: input.into_iter().map(Vec::from).collect(),
            positions_xmas,
            positions_x_mas,
            result_part1,
            result_part2,
        }
    }

    fn test_left(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if input
            .get(position.1)
            .and_then(|line| line.get(position.0 + 1))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_left(input, (position.0 + 1, position.1), &rest[1..])
        } else {
            false
        }
    }

    fn test_right(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if input
            .get(position.1)
            .and_then(|line| position.0.checked_sub(1).and_then(|x| line.get(x)))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_right(input, (position.0 - 1, position.1), &rest[1..])
        } else {
            false
        }
    }

    fn test_up(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if position
            .1
            .checked_sub(1)
            .and_then(|y| input.get(y))
            .and_then(|line| line.get(position.0))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_up(input, (position.0, position.1 - 1), &rest[1..])
        } else {
            false
        }
    }

    fn test_down(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if input
            .get(position.1 + 1)
            .and_then(|line| line.get(position.0))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_down(input, (position.0, position.1 + 1), &rest[1..])
        } else {
            false
        }
    }

    fn test_ne(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if position
            .1
            .checked_sub(1)
            .and_then(|y| input.get(y))
            .and_then(|line| line.get(position.0 + 1))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_ne(input, (position.0 + 1, position.1 - 1), &rest[1..])
        } else {
            false
        }
    }

    fn test_se(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if input
            .get(position.1 + 1)
            .and_then(|line| line.get(position.0 + 1))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_se(input, (position.0 + 1, position.1 + 1), &rest[1..])
        } else {
            false
        }
    }

    fn test_nw(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if position
            .1
            .checked_sub(1)
            .and_then(|y| input.get(y))
            .and_then(|line| position.0.checked_sub(1).and_then(|x| line.get(x)))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_nw(input, (position.0 - 1, position.1 - 1), &rest[1..])
        } else {
            false
        }
    }

    fn test_sw(input: &[&[u8]], position: (usize, usize), rest: &[u8]) -> bool {
        if rest.is_empty() {
            true
        } else if input
            .get(position.1 + 1)
            .and_then(|line| position.0.checked_sub(1).and_then(|x| line.get(x)))
            .filter(|c| **c == rest[0])
            .is_some()
        {
            Self::test_sw(input, (position.0 - 1, position.1 + 1), &rest[1..])
        } else {
            false
        }
    }
}
