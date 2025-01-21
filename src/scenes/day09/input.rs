use bevy::{color::Color, prelude::Resource};

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub start: Vec<Block>,
    pub disk: Vec<Block>,
    pub pos: usize,
    last_moved: u64,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let line = input.split(|c| *c == b'\n').next().unwrap();

        let mut unique = 0;

        let mut is_block = true;
        let blocks = line
            .iter()
            .map(|block_size| usize::from(block_size - b'0'))
            .flat_map(|block_size| {
                let id_block = is_block;
                is_block = !is_block;
                if id_block {
                    unique += 1;
                    let id = unique;
                    vec![id; block_size]
                } else {
                    vec![0; block_size]
                }
            })
            // the `flat_map` must run to completion before running the `map`
            .collect::<Vec<_>>()
            .into_iter()
            .map(|block_id| {
                let color = if block_id == 0 {
                    Color::BLACK
                } else {
                    let hue = (block_id as f32 / unique as f32) * 180.;
                    Color::hsv(hue, 1., 1.).to_linear().into()
                };
                Block(block_id, color)
            })
            .collect::<Vec<_>>();

        let blocks_len = blocks.len();

        Self {
            start: blocks.clone(),
            disk: blocks,
            pos: blocks_len,
            last_moved: unique + 1,
        }
    }

    pub fn reset(&mut self) {
        self.disk = self.start.clone();
        self.pos = self.disk.len();
        self.last_moved = self.disk[self.disk.len() - 1].0 + 1;
    }

    pub fn calculate_checksum(&self) -> u64 {
        self.disk
            .iter()
            .enumerate()
            .filter(|(_, block)| block.0 != 0)
            .map(|(i, block)| u64::try_from(i).unwrap() * (block.0 - 1))
            .sum()
    }

    pub fn defrag_single(&mut self) {
        if self.pos == 0 {
            return;
        }

        self.pos -= 1;
        while self.disk[self.pos].0 == 0 {
            self.pos -= 1;
        }

        let empty = self.disk.iter().position(|block| block.0 == 0).unwrap();

        if self.pos > empty {
            self.disk.swap(self.pos, empty);
        } else {
            self.pos = 0;
        }
    }

    pub fn defrag_multi(&mut self) {
        if self.pos == 0 {
            return;
        }

        self.pos -= 1;
        while self.disk[self.pos].0 == 0 {
            self.pos -= 1;
        }
        let end = self.pos;
        let block_id = self.disk[end].0;

        while self.disk[self.pos].0 == block_id {
            if self.pos == 0 {
                return;
            }
            self.pos -= 1;
        }
        self.pos += 1;
        let start = self.pos;

        let mut empty_start = 0;
        let mut empty_end;
        loop {
            while self.disk[empty_start].0 != 0 {
                empty_start += 1;
            }
            empty_end = empty_start;

            while self.disk[empty_end].0 == 0 {
                empty_end += 1;
                if empty_end >= self.disk.len() {
                    return;
                }
            }
            empty_end -= 1;

            if empty_end >= start {
                return;
            } else if empty_end - empty_start >= end - start {
                break;
            } else {
                empty_start = empty_end + 1;
            }
        }

        if block_id < self.last_moved {
            for (x, y) in (empty_start..=empty_end).zip(start..=end) {
                self.last_moved = block_id;
                self.disk.swap(x, y);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block(pub u64, pub Color);
