use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

use bevy::prelude::{Component, Resource};

use crate::{
    loader::RawInput,
    tools::{Coord, Maze, Vec2d},
};

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub data: Vec<u8>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        Self {
            data: input.0.clone(),
        }
    }

    pub fn cheat(&mut self, cheat_len: usize) -> BTreeMap<Coord, Vec<(Coord, usize)>> {
        let maze = Maze::parse(&mut self.data, 0);

        let (mut tile_cost_data, main_path) = maze.calculate_tile_scores();
        let tile_cost = Vec2d::new(tile_cost_data.as_mut_slice(), maze.width(), maze.height());

        main_path.iter().fold(BTreeMap::new(), |mut map, start| {
            map.insert(
                *start,
                Vec::from_iter(Self::find_reachable(*start, &tile_cost, cheat_len)),
            );
            map
        })
    }

    fn find_reachable(
        from: Coord,
        tile_cost: &Vec2d<usize>,
        cheat_len: usize,
    ) -> BTreeSet<(Coord, usize)> {
        assert_ne!(tile_cost[from], usize::MAX);

        let mut set = BTreeSet::new();

        let bounds = Coord::new(tile_cost.height(), tile_cost.width());

        let cheat_len_i = isize::try_from(cheat_len).unwrap();

        for (row_offset, column_offset) in ((-cheat_len_i)..=(cheat_len_i)).flat_map(|row_offset| {
            let remaining_for_column = isize::try_from(
                cheat_len_i
                    .abs_diff(row_offset)
                    .min(cheat_len_i.abs_diff(-row_offset)),
            )
            .unwrap();
            ((-remaining_for_column)..=(remaining_for_column))
                .map(move |column_offset| (row_offset, column_offset))
        }) {
            let Some(no_clip) = from.row.checked_add_signed(row_offset).and_then(|row| {
                from.column
                    .checked_add_signed(column_offset)
                    .map(|column| Coord::new(row, column))
                    .filter(|coord| coord.row < bounds.row && coord.column < bounds.column)
            }) else {
                continue;
            };
            if from != no_clip
                && tile_cost[from] < tile_cost[no_clip]
                && tile_cost[no_clip] != usize::MAX
            {
                let clipped = row_offset.unsigned_abs() + column_offset.unsigned_abs();
                let short_cut = tile_cost[no_clip] - tile_cost[from] - clipped;
                if short_cut > 0 {
                    set.insert((no_clip, short_cut));
                }
            }
        }

        set
    }
}
