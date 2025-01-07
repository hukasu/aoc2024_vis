use std::io::BufRead;

use bevy::prelude::Resource;

use crate::loader::Input as InputAsset;

#[derive(Debug, Resource)]
pub struct Input {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
}

impl Input {
    pub fn parse(input: &InputAsset) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in input.lines() {
            let line = line.unwrap();

            let (l, r) = line.split_once("   ").unwrap();

            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }

        Self { left, right }
    }
}
