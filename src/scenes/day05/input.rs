use std::{cmp::Ordering, collections::BTreeMap};

use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub manuals: Vec<Manual>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let mut rules = BTreeMap::new();
        let mut manuals = Vec::new();

        let mut lines = input.0.split(|c| *c == b'\n');

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let (l, r) = line.split_at(2);

            let l = String::from_utf8_lossy(l)
                .parse()
                .expect("Left value should be valid number");
            let r = String::from_utf8_lossy(&r[1..])
                .parse()
                .expect("Right value should be valid number");

            rules
                .entry(l)
                .and_modify(|vec: &mut Vec<u32>| {
                    vec.push(r);
                })
                .or_insert(vec![r]);
        }

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let mut pages = line
                .split(|c| *c == b',')
                .map(String::from_utf8_lossy)
                .map(|val| val.parse())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let sorted = pages.is_sorted_by(Self::comparator(&rules));

            if !sorted {
                pages.sort_by(Self::sorter(&rules));
            }

            manuals.push(Manual { sorted, pages });
        }

        Self { manuals }
    }

    fn comparator(map: &BTreeMap<u32, Vec<u32>>) -> impl FnMut(&u32, &u32) -> bool + use<'_> {
        |l, r| map.get(l).expect("Page must exist.").contains(r)
    }

    fn sorter(map: &BTreeMap<u32, Vec<u32>>) -> impl FnMut(&u32, &u32) -> Ordering + use<'_> {
        |l, r| {
            if map.get(l).expect("Page must exist.").contains(r) {
                Ordering::Less
            } else if map.get(r).expect("Page must exist.").contains(l) {
                Ordering::Greater
            } else {
                unreachable!("Every pair should exists.");
            }
        }
    }
}

#[derive(Debug)]
pub struct Manual {
    pub sorted: bool,
    pub pages: Vec<u32>,
}
