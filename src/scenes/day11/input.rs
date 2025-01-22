use std::collections::BTreeMap;

use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub start: Vec<u64>,
    pub pebbles: BTreeMap<u64, usize>,
    pub twenty_five: usize,
    pub seventy_five: usize,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let start = input
            .split(|c| *c == b'\n')
            .next()
            .unwrap()
            .split(|c| *c == b' ')
            .map(|val| String::from_utf8_lossy(val).parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut input = Self {
            start,
            pebbles: BTreeMap::new(),
            twenty_five: 0,
            seventy_five: 0,
        };
        input.reset();

        for _ in 0..25 {
            input.blink();
        }

        input.twenty_five = input.pebbles.values().sum();

        for _ in 0..50 {
            input.blink();
        }

        input.seventy_five = input.pebbles.values().sum();

        input.reset();

        input
    }

    pub fn reset(&mut self) {
        self.pebbles.clear();
        for pebble in self.start.iter() {
            self.pebbles
                .entry(*pebble)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }
    }

    pub fn blink(&mut self) {
        let pebbles = std::mem::take(&mut self.pebbles);
        for (pebble, count) in pebbles {
            match pebble {
                0 => {
                    self.pebbles
                        .entry(1)
                        .and_modify(|pebble_count| *pebble_count += count)
                        .or_insert(count);
                }
                n if n.ilog10() % 2 == 1 => {
                    let digit_count = n.ilog10() + 1;

                    self.pebbles
                        .entry(n % (10u64.pow(digit_count / 2)))
                        .and_modify(|pebble_count| *pebble_count += count)
                        .or_insert(count);
                    self.pebbles
                        .entry(n / (10u64.pow(digit_count / 2)))
                        .and_modify(|pebble_count| *pebble_count += count)
                        .or_insert(count);
                }
                n => {
                    self.pebbles
                        .entry(n * 2024)
                        .and_modify(|pebble_count| *pebble_count += count)
                        .or_insert(count);
                }
            }
        }
    }
}
