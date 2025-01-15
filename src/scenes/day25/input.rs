use bevy::prelude::Resource;

use crate::loader::RawInput;

use super::components::{Key, Lock};

#[derive(Debug, Resource)]
pub struct Input {
    pub keys: Vec<Key>,
    pub locks: Vec<Lock>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let lines = input.split(|c| *c == b'\n');

        let mut keys = Vec::new();
        let mut locks = Vec::new();

        let mut reading_lock = false;
        let mut temp = None;

        for line in lines {
            if line.is_empty() {
                if reading_lock {
                    let pins: (u8, u8, u8, u8, u8) = temp.take().unwrap();
                    locks.push(Lock(pins.0, pins.1, pins.2, pins.3, pins.4));
                } else {
                    let pins = temp.take().unwrap();
                    keys.push(Key(pins.0, pins.1, pins.2, pins.3, pins.4));
                }
            } else {
                let row = line
                    .iter()
                    .enumerate()
                    .fold((0, 0, 0, 0, 0), |mut accum, (i, c)| {
                        if *c == b'#' {
                            match i {
                                0 => accum.0 = 1,
                                1 => accum.1 = 1,
                                2 => accum.2 = 1,
                                3 => accum.3 = 1,
                                4 => accum.4 = 1,
                                _ => unreachable!("Line too long"),
                            };
                        }
                        accum
                    });

                if temp.is_none() {
                    reading_lock = row == (1, 1, 1, 1, 1);
                    temp.replace(row);
                } else {
                    temp = temp.map(|accum| {
                        (
                            accum.0 + row.0,
                            accum.1 + row.1,
                            accum.2 + row.2,
                            accum.3 + row.3,
                            accum.4 + row.4,
                        )
                    });
                }
            }
        }

        Self { keys, locks }
    }
}
