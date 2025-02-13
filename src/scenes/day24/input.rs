use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::{Component, Resource};

use crate::loader::RawInput;

use super::operation::{Operation, Operator};

#[derive(Debug, Clone, Component)]
pub enum ExecutionResult {
    Success([u8; 3], [u8; 3], [u8; 3]),
    Failure([u8; 3], [u8; 3]),
}

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub x: [u8; 45],
    pub y: [u8; 45],
    pub z: [u8; 46],
    pub intermediate: BTreeMap<[u8; 3], u8>,
    pub operations: Vec<Operation>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
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

        let mut carry = [0; 3];

        for index in 0u8..45 {
            let inputs = [
                [b'x', (index / 10) + b'0', (index % 10) + b'0'],
                [b'y', (index / 10) + b'0', (index % 10) + b'0'],
            ];
            set.extend(inputs);

            let xor = {
                let xor_pos = operations
                    .iter()
                    .position(|op| {
                        inputs.contains(&op.l)
                            && inputs.contains(&op.r)
                            && matches!(op.operator, Operator::Xor)
                    })
                    .unwrap();
                operations.remove(xor_pos)
            };
            let and = {
                let and_pos = operations
                    .iter()
                    .position(|op| {
                        inputs.contains(&op.l)
                            && inputs.contains(&op.r)
                            && matches!(op.operator, Operator::And)
                    })
                    .unwrap();
                operations.remove(and_pos)
            };

            input.operations.push(xor.clone());
            input.operations.push(and.clone());
            set.insert(xor.out);
            set.insert(and.out);

            if index != 0 {
                let inputs = [xor.out, carry];

                let xor_b = {
                    let xor_pos = operations
                        .iter()
                        .position(|op| {
                            inputs.contains(&op.l)
                                && inputs.contains(&op.r)
                                && matches!(op.operator, Operator::Xor)
                        })
                        .or_else(|| {
                            operations.iter().position(|op| {
                                set.contains(&op.l)
                                    && set.contains(&op.r)
                                    && matches!(op.operator, Operator::Xor)
                            })
                        })
                        .unwrap();
                    operations.remove(xor_pos)
                };
                let and_b = {
                    let and_pos = operations
                        .iter()
                        .position(|op| {
                            inputs.contains(&op.l)
                                && inputs.contains(&op.r)
                                && matches!(op.operator, Operator::And)
                        })
                        .or_else(|| {
                            operations.iter().position(|op| {
                                set.contains(&op.l)
                                    && set.contains(&op.r)
                                    && matches!(op.operator, Operator::And)
                            })
                        })
                        .unwrap();
                    operations.remove(and_pos)
                };

                input.operations.push(xor_b.clone());
                input.operations.push(and_b.clone());
                set.insert(xor_b.out);
                set.insert(and_b.out);

                let inputs = [and.out, and_b.out];
                let or = {
                    let or_pos = operations
                        .iter()
                        .position(|op| {
                            inputs.contains(&op.l)
                                && inputs.contains(&op.r)
                                && matches!(op.operator, Operator::Or)
                        })
                        .or_else(|| {
                            operations.iter().position(|op| {
                                set.contains(&op.l)
                                    && set.contains(&op.r)
                                    && matches!(op.operator, Operator::Or)
                            })
                        })
                        .unwrap();
                    operations.remove(or_pos)
                };

                input.operations.push(or.clone());
                set.insert(or.out);

                carry = or.out;
            }
        }

        assert!(operations.is_empty());

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
            ExecutionResult::Success(top.l, top.r, top.out)
        } else {
            self.operations.push(top.clone());
            ExecutionResult::Failure(top.l, top.r)
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
