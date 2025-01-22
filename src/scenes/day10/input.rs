use std::collections::{BTreeMap, VecDeque};

use bevy::prelude::Resource;

use crate::loader::RawInput;

type Coord = (usize, usize);

#[derive(Debug, Resource)]
pub struct Input {
    pub tiles: Vec<Vec<u8>>,
    pub trails: BTreeMap<Coord, Vec<Vec<Coord>>>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let tiles = input
            .into_iter()
            .map(|line| line.iter().map(|tile| tile - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut trails = BTreeMap::new();

        let mut queue = VecDeque::from_iter(tiles.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, tile)| {
                if *tile == 0 {
                    Some(((x, y), (x, y), vec![]))
                } else {
                    None
                }
            })
        }));

        while let Some((start, coord, trail)) = queue.pop_front() {
            let tile = tiles[coord.1][coord.0];
            if tile == 9 {
                trails
                    .entry(start)
                    .and_modify(|trails: &mut Vec<_>| trails.push(trail.clone()))
                    .or_insert(vec![trail]);
            } else {
                let adjacent = [
                    coord.0.checked_sub(1).map(|x| (x, coord.1)),
                    Some(coord.0 + 1)
                        .filter(|x| *x < tiles[0].len())
                        .map(|x| (x, coord.1)),
                    coord.1.checked_sub(1).map(|y| (coord.0, y)),
                    Some(coord.1 + 1)
                        .filter(|y| *y < tiles.len())
                        .map(|y| (coord.0, y)),
                ];
                for adjacent in adjacent.into_iter().flatten() {
                    if tiles[adjacent.1][adjacent.0] == tile + 1 {
                        let mut new_trail = trail.clone();
                        new_trail.push(adjacent);
                        queue.push_back((start, adjacent, new_trail));
                    }
                }
            }
        }

        assert!(trails
            .iter()
            .all(|trails_from_head| trails_from_head.1.iter().all(|trail| trail.len() == 9)));

        Self { tiles, trails }
    }
}
