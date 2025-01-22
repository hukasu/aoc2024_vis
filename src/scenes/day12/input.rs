use bevy::prelude::Resource;

use crate::{
    loader::RawInput,
    tools::{Convolution, MergeInsert, MergeInsertNode},
};

#[derive(Debug, Resource)]
pub struct Input {
    pub tiles: Vec<Vec<u8>>,
    pub fences: u64,
    pub fence_runs: u64,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let tiles = input
            .into_iter()
            .map(|line| line.to_vec())
            .collect::<Vec<_>>();

        let slices = tiles.iter().map(AsRef::as_ref).collect::<Vec<_>>();

        let fences = Self::calculate_area_perimeter(slices.as_slice())
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum();

        let fence_runs = Self::calculate_sides(slices.as_slice())
            .into_iter()
            .map(|(area, perimeter)| area * perimeter)
            .sum();

        Self {
            tiles,
            fences,
            fence_runs,
        }
    }

    fn calculate_area_perimeter(data: &[&[u8]]) -> Vec<(u64, u64)> {
        let mut merge_insert = MergeInsert::default();

        for convolution in Convolution::new(data) {
            let ([[_, t, _], [l, c, r], [_, b, _]], line, col) = convolution;

            let cur_area = 1u64;
            let cur_perimeter =
                (4 - [t, r, b, l].into_iter().filter(|crop| *crop == c).count()) as u64;

            match (c == l, c == t) {
                (true, true) => {
                    merge_insert.insert((line, col), (cur_area, cur_perimeter));
                    merge_insert.merge((line, col), (line, col - 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                (true, false) => {
                    merge_insert.insert((line, col), (cur_area, cur_perimeter));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                (false, true) => {
                    merge_insert.insert((line, col), (cur_area, cur_perimeter));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                (false, false) => {
                    merge_insert.insert((line, col), (cur_area, cur_perimeter));
                }
            }
        }

        merge_insert
            .drain()
            .filter_map(|(_, v)| match v {
                MergeInsertNode::Root(v) => Some(v.get()),
                _ => None,
            })
            .collect()
    }

    fn calculate_sides(data: &[&[u8]]) -> Vec<(u64, u64)> {
        let mut merge_insert = MergeInsert::default();

        for (convolution, line, col) in Convolution::new(data) {
            let c = convolution[1][1];
            match convolution.map(|line| line.map(|col| col == c)) {
                // Unreacheable
                [_, [_, false, _], _] => unreachable!("Center is always equal to center."),
                // No adjacents
                [[_, false, _], [false, true, false], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 4));
                }
                // Four way adjacents
                [[_, true, _], [true, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // XAX
                // XXX
                [[false, true, false], [false, true, false], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAX
                // XAX
                // XXX
                [[true, true, false], [false, true, false], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XAA
                // XAX
                // XXX
                [[false, true, true], [false, true, false], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAA
                // XAX
                // XXX
                [[true, true, true], [false, true, false], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 3));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XXX
                // AAX
                // XXX
                [[false, false, _], [true, true, false], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAX
                // XXX
                [[true, false, _], [true, true, false], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XXX
                // AAX
                // AXX
                [[false, false, _], [true, true, false], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAX
                // AXX
                [[true, false, _], [true, true, false], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 3));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XXX
                // XAX
                // XAX
                [[_, false, _], [false, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 3));
                }
                // XXX
                // XAA
                // XXX
                [[_, false, _], [false, true, true], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 3));
                }
                // XAX
                // AAX
                // XXX
                [[_, true, false], [true, true, false], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAA
                // AAX
                // XXX
                [[_, true, true], [true, true, false], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // AAX
                // AXX
                [[_, true, false], [true, true, false], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAA
                // AAX
                // AXX
                [[_, true, true], [true, true, false], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // XAX
                // XAX
                [[false, true, false], [false, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAX
                // XAX
                // XAX
                [[true, true, false], [false, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XAA
                // XAX
                // XAX
                [[false, true, true], [false, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAA
                // XAX
                // XAX
                [[true, true, true], [false, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XAX
                // XAA
                // XXX
                [[false, true, _], [false, true, true], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAX
                // XAA
                // XXX
                [[true, true, _], [false, true, true], [_, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XXX
                // AAX
                // XAX
                [[false, false, _], [true, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAX
                // XAX
                [[true, false, _], [true, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XXX
                // AAA
                // XXX
                [[false, false, _], [true, true, true], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAA
                // XXX
                [[true, false, _], [true, true, true], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XXX
                // AAA
                // AXX
                [[false, false, _], [true, true, true], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAA
                // AXX
                [[true, false, _], [true, true, true], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XXX
                // XAA
                // XAX
                [[_, false, _], [false, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 2));
                }
                // XAX
                // AAX
                // XAX
                [[_, true, false], [true, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAA
                // AAX
                // XAX
                [[_, true, true], [true, true, false], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // AAA
                // XXX
                [[_, true, _], [true, true, true], [false, false, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // AAA
                // AXX
                [[_, true, _], [true, true, true], [true, false, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // XAX
                // XAA
                // XAX
                [[false, true, _], [false, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // AAX
                // XAA
                // XAX
                [[true, true, _], [false, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line - 1, col));
                }
                // XXX
                // AAA
                // XAX
                [[false, false, _], [true, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 0));
                    merge_insert.merge((line, col), (line, col - 1));
                }
                // AXX
                // AAA
                // XAX
                [[true, false, _], [true, true, true], [_, true, _]] => {
                    merge_insert.insert((line, col), (1, 1));
                    merge_insert.merge((line, col), (line, col - 1));
                }
            };
        }

        merge_insert
            .drain()
            .filter_map(|(_, v)| match v {
                MergeInsertNode::Root(v) => Some(v.get()),
                _ => None,
            })
            .collect()
    }
}
