use bevy::prelude::Resource;

use crate::loader::RawInput;

#[derive(Debug, Resource)]
pub struct Input {
    pub two_ops: u64,
    pub three_ops: u64,
    pub operations: Vec<Operation>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let mut two_ops = 0;
        let mut three_ops = 0;

        let mut operations = vec![];

        for line in input {
            let line_split = line.split(|c| *c == b':').collect::<Vec<_>>();
            let [l, r] = line_split.as_slice() else {
                unreachable!("There should be 1 ':'.");
            };

            let result = String::from_utf8_lossy(l).parse().unwrap();
            let operands: Vec<u64> = r
                .split(|c| *c == b' ')
                .filter(|val| !val.is_empty())
                .map(|val| String::from_utf8_lossy(val).parse().unwrap())
                .collect();
            let operators = Self::is_valid_operation(
                result,
                operands.as_slice(),
                &[
                    ("+", std::ops::Add::add),
                    ("*", std::ops::Mul::mul),
                    ("||", concatenate_u64),
                ],
            )
            .into_iter()
            .map(|(operator, _)| operator)
            .collect::<Vec<_>>();

            if !operators.is_empty() {
                three_ops += result;
                if !operators.contains(&"||") {
                    two_ops += result;
                }
            }

            operations.push(Operation {
                result,
                operands,
                operators,
            });
        }

        Self {
            two_ops,
            three_ops,
            operations,
        }
    }

    fn is_valid_operation(result: u64, operands: &[u64], operators: &[Operator]) -> Vec<Operator> {
        let mut permutations = Permutations::new(operands.len() - 1, operators);
        permutations
            .find(|permutation: &Vec<Operator>| {
                let mut permutation = permutation.iter();
                let operation = operands[1..].iter().fold(operands[0], |sum, operand| {
                    permutation.next().unwrap().1(sum, *operand)
                });
                operation == result
            })
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct Operation {
    pub result: u64,
    pub operands: Vec<u64>,
    pub operators: Vec<&'static str>,
}

type Operator = (&'static str, fn(u64, u64) -> u64);

struct Permutations<'a> {
    cur: Vec<usize>,
    operators: &'a [Operator],
    iterations: usize,
}

impl<'a> Permutations<'a> {
    fn new(len: usize, operators: &'a [Operator]) -> Self {
        Self {
            cur: vec![0; len],
            operators,
            iterations: 0,
        }
    }
}

impl Iterator for Permutations<'_> {
    type Item = Vec<(&'static str, fn(u64, u64) -> u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations >= self.operators.len().pow(self.cur.len() as u32) {
            return None;
        }
        let mut to_mutate = 0;
        while let Some(mutate) = self.cur.get_mut(to_mutate) {
            if *mutate == self.operators.len() - 1 {
                *mutate = 0;
                to_mutate += 1;
            } else {
                *mutate += 1;
                to_mutate = usize::MAX;
            }
        }

        self.iterations += 1;
        Some(
            self.cur
                .iter()
                .map(|index| self.operators[*index])
                .collect(),
        )
    }
}

fn concatenate_u64(lhs: u64, rhs: u64) -> u64 {
    let rhs_log = rhs.ilog10();
    lhs * 10u64.pow(rhs_log + 1) + rhs
}
