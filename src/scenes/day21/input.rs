use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
};

use bevy::prelude::{Component, Resource};

use crate::{loader::RawInput, tools::Coord};

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub codes: [Code; 5],
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let codes = input
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| Code(line.try_into().unwrap()))
            .collect::<Vec<_>>();
        let codes = codes.try_into().unwrap();

        Self { codes }
    }

    pub fn run(&self, indirections: usize) -> (Vec<usize>, BTreeMap<(u8, u8), usize>) {
        let numeric = KeyPad::numeric();
        let directional = KeyPad::directional();

        let mut top_level = BTreeMap::new();

        let mut cache = BTreeMap::new();

        for ((l, r), paths) in &numeric.paths {
            let min = paths
                .iter()
                .map(|path| {
                    Self::indirection(path.as_slice(), &directional, indirections, &mut cache)
                })
                .min()
                .unwrap();

            top_level
                .entry((*l, *r))
                .and_modify(|score| {
                    if *score > min {
                        *score = min;
                    }
                })
                .or_insert(min);
        }

        (
            self.codes
                .into_iter()
                .map(|code| {
                    code.0
                        .into_iter()
                        .scan(b'A', |prev, cur| {
                            let min = top_level.get(&(*prev, cur)).unwrap();
                            *prev = cur;

                            Some(*min)
                        })
                        .sum::<usize>()
                        * code.parse_code()
                })
                .collect(),
            top_level,
        )
    }

    fn indirection<'a>(
        segment: &'a [u8],
        directional: &'a KeyPad,
        indirections: usize,
        cache: &mut BTreeMap<(usize, &'a [u8]), usize>,
    ) -> usize {
        assert_eq!(segment[segment.len() - 1], b'A');
        if indirections == 0 {
            segment.len()
        } else if let Some(cached) = cache.get(&(indirections, segment)) {
            *cached
        } else {
            let segment_score = segment
                .iter()
                .scan(b'A', |prev, cur| {
                    let next_segments = directional.paths.get(&(*prev, *cur)).unwrap();
                    *prev = *cur;
                    next_segments
                        .iter()
                        .map(|segment| {
                            Self::indirection(segment, directional, indirections - 1, cache)
                        })
                        .min()
                })
                .sum();
            cache.insert((indirections, segment), segment_score);
            segment_score
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Code(pub [u8; 4]);

impl Code {
    fn parse_code(self) -> usize {
        String::from_utf8_lossy(&self.0[..3]).parse().unwrap()
    }
}

#[derive(Debug)]
struct KeyPad {
    paths: BTreeMap<(u8, u8), Vec<Vec<u8>>>,
}

impl KeyPad {
    fn build_from_keys(keys: &[(u8, Coord)]) -> Self {
        let error = keys
            .iter()
            .find_map(|(key, coord)| if *key == 0 { Some(*coord) } else { None })
            .unwrap();
        Self {
            paths: keys
                .iter()
                .filter(|(key, _)| *key != 0)
                .flat_map(|(l, l_coord)| {
                    keys.iter()
                        .filter(|(key, _)| *key != 0)
                        .map(|(r, r_coord)| {
                            (
                                (*l, *r),
                                if *r == 0 {
                                    vec![]
                                } else {
                                    Self::possible_paths_between_keys(*l_coord, *r_coord, error)
                                },
                            )
                        })
                })
                .collect(),
        }
    }

    fn numeric() -> Self {
        Self::build_from_keys(&[
            (0, Coord::new(3, 0)),
            (b'0', Coord::new(3, 1)),
            (b'A', Coord::new(3, 2)),
            (b'1', Coord::new(2, 0)),
            (b'2', Coord::new(2, 1)),
            (b'3', Coord::new(2, 2)),
            (b'4', Coord::new(1, 0)),
            (b'5', Coord::new(1, 1)),
            (b'6', Coord::new(1, 2)),
            (b'7', Coord::new(0, 0)),
            (b'8', Coord::new(0, 1)),
            (b'9', Coord::new(0, 2)),
        ])
    }

    fn directional() -> Self {
        Self::build_from_keys(&[
            (0, Coord::new(0, 0)),
            (b'^', Coord::new(0, 1)),
            (b'A', Coord::new(0, 2)),
            (b'<', Coord::new(1, 0)),
            (b'v', Coord::new(1, 1)),
            (b'>', Coord::new(1, 2)),
        ])
    }

    fn possible_paths_between_keys(start: Coord, end: Coord, error: Coord) -> Vec<Vec<u8>> {
        let mut result = vec![];
        let mut queue = VecDeque::with_capacity(50);
        queue.push_back((start, vec![]));

        while let Some((cur, path)) = queue.pop_front() {
            if cur == error {
                continue;
            } else if cur == end {
                let mut new_path = path.clone();
                new_path.push(b'A');
                result.push(new_path);
            } else {
                if cur.row > end.row {
                    let mut new_path = path.clone();
                    new_path.push(b'^');
                    queue.push_back((cur - (1, 0), new_path));
                }
                if cur.row < end.row {
                    let mut new_path = path.clone();
                    new_path.push(b'v');
                    queue.push_back((cur + (1, 0), new_path));
                }
                if cur.column > end.column {
                    let mut new_path = path.clone();
                    new_path.push(b'<');
                    queue.push_back((cur - (0, 1), new_path));
                }
                if cur.column < end.column {
                    let mut new_path = path.clone();
                    new_path.push(b'>');
                    queue.push_back((cur + (0, 1), new_path));
                }
            }
        }

        result
    }
}
