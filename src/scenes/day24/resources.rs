use std::collections::{BTreeMap, BTreeSet};

use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::loader::Input as InputAsset;

use super::operation::{Operation, Operator};

#[derive(Debug, Resource)]
pub struct Day24 {
    pub input: Handle<InputAsset>,
    pub camera: Entity,
    pub ui: Entity,
}

pub type ExecutionResult = Result<([u8; 3], [u8; 3], [u8; 3]), ([u8; 3], [u8; 3])>;

#[derive(Debug, Resource)]
pub struct Input {
    pub x: [u8; 45],
    pub y: [u8; 45],
    pub z: [u8; 46],
    pub intermediate: BTreeMap<[u8; 3], u8>,
    pub operations: Vec<Operation>,
}

impl Input {
    pub fn parse(input: &InputAsset) -> Self {
        let mut lines = input.split(|c| *c == b'\n');

        let mut input = Input::default();

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let (l, r) = line.split_at(3);
            match l {
                [b'x', d, u] => input.x[usize::from(ascii_to_num(*d, *u))] = r[2] - b'0',
                [b'y', d, u] => input.y[usize::from(ascii_to_num(*d, *u))] = r[2] - b'0',
                _ => unreachable!("First section must only contain x and y."),
            }
        }

        for line in lines.filter(|line| !line.is_empty()) {
            let line = line.split(|c| *c == b' ').collect::<Vec<_>>();
            let [l, op, r, _, out] = line.as_slice() else {
                unreachable!("Line on second section must have 5 items.");
            };

            let op = match op {
                [b'A', b'N', b'D'] => Operator::And,
                [b'O', b'R'] => Operator::Or,
                [b'X', b'O', b'R'] => Operator::Xor,
                _ => unreachable!("Invalid operator"),
            };

            let l: [u8; 3] = (*l).try_into().expect("Should be able to convert");
            if !matches!(l, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                input.intermediate.insert(l, u8::MAX);
            }
            let r: [u8; 3] = (*r).try_into().expect("Should be able to convert");
            if !matches!(r, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                input.intermediate.insert(r, u8::MAX);
            }
            let out: [u8; 3] = (*out).try_into().expect("Should be able to convert");
            if !matches!(out, [b'x', _, _] | [b'y', _, _] | [b'z', _, _]) {
                input.intermediate.insert(out, u8::MAX);
            }

            input.operations.push(Operation {
                l,
                operator: op,
                r,
                out,
            });
        }

        let mut operations = std::mem::take(&mut input.operations);

        let mut set = BTreeSet::new();
        set.insert([b'x', b'0', b'0']);
        set.insert([b'y', b'0', b'0']);

        let mut index = 0;
        let mut count = 3;

        while !operations.is_empty() {
            let op = operations.remove(0);

            if set.contains(&op.l) && set.contains(&op.r) {
                set.insert(op.out);
                input.operations.push(op);
                count += 1;
                if count >= 5 {
                    count = 0;
                    index += 1;

                    set.insert([b'x', (index / 10) + b'0', (index % 10) + b'0']);
                    set.insert([b'y', (index / 10) + b'0', (index % 10) + b'0']);
                }
            } else {
                operations.push(op);
            }
        }

        input
    }

    pub fn run_program(&mut self) {
        while !self.operations.is_empty() {
            let _ = self.execute_top();
        }
    }

    pub fn execute_top(&mut self) -> ExecutionResult {
        let top = self.operations.remove(0);

        if let (Some(l), Some(r)) = (self.get(top.l), self.get(top.r)) {
            let res = top.operator.func()(l, r);
            self.set(top.out, res);
            Ok((top.l, top.r, top.out))
        } else {
            self.operations.push(top.clone());
            Err((top.l, top.r))
        }
    }

    fn get(&self, key: [u8; 3]) -> Option<u8> {
        match key {
            [b'x', a, b] => Some(self.x[usize::from((a - b'0') * 10 + b - b'0')]),
            [b'y', a, b] => Some(self.y[usize::from((a - b'0') * 10 + b - b'0')]),
            [b'z', a, b] => Some(self.z[usize::from((a - b'0') * 10 + b - b'0')]),
            other => self
                .intermediate
                .get(&other)
                .filter(|val| **val != u8::MAX)
                .copied(),
        }
    }

    fn set(&mut self, key: [u8; 3], value: u8) {
        match key {
            [b'x' | b'y', _, _] => unreachable!("x and y are immutable"),
            [b'z', a, b] => {
                self.z[usize::from((a - b'0') * 10 + b - b'0')] = value;
            }
            other => {
                self.intermediate
                    .entry(other)
                    .and_modify(|val| *val = value);
            }
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            x: [0; 45],
            y: [0; 45],
            z: [0; 46],
            intermediate: BTreeMap::new(),
            operations: Vec::new(),
        }
    }
}

fn ascii_to_num(d: u8, u: u8) -> u8 {
    (d - b'0') * 10 + (u - b'0')
}
