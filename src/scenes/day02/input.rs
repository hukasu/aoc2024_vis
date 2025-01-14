use std::{cmp::Ordering, io::BufRead};

use bevy::prelude::Resource;

use crate::loader::Input as InputAsset;

#[derive(Debug, Resource)]
pub struct Input {
    pub reports: Vec<Report>,
}

#[derive(Debug)]
pub struct Report {
    pub report: Vec<u8>,
    pub safety: Safety,
}

#[derive(Debug)]
pub enum Safety {
    Safe,
    OneError(usize),
    Unsafe,
}

#[derive(Debug, Default)]
struct SafetyTest {
    previous: u8,
    increasing: bool,
    decreasing: bool,
    stagnant: bool,
    max_change: u8,
}

impl Input {
    pub fn parse(input: &InputAsset) -> Self {
        let mut reports = Vec::new();

        for line in input.lines() {
            let line = line.unwrap();

            let report = line
                .split_ascii_whitespace()
                .map(|val| val.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            let safety = if Self::test_safety(report.iter().copied()) {
                Safety::Safe
            } else if let Some(pos) = (0..report.len()).position(|i| {
                Self::test_safety(
                    report
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, val)| *val),
                )
            }) {
                Safety::OneError(pos)
            } else {
                Safety::Unsafe
            };

            reports.push(Report { report, safety });
        }

        Self { reports }
    }

    fn test_safety(levels: impl Iterator<Item = u8>) -> bool {
        let safety = levels.fold(SafetyTest::default(), |mut safety, level| {
            if safety.previous == 0 {
                safety.previous = level;
            } else {
                let cmp = safety.previous.cmp(&level);
                safety.increasing = safety.increasing || cmp == Ordering::Less;
                safety.decreasing = safety.decreasing || cmp == Ordering::Greater;
                safety.stagnant = safety.stagnant || cmp == Ordering::Equal;
                safety.max_change = safety.max_change.max(safety.previous.abs_diff(level));
                safety.previous = level;
            }
            safety
        });
        ((safety.increasing && !safety.decreasing) || (!safety.increasing && safety.decreasing))
            && !safety.stagnant
            && (1..=3).contains(&safety.max_change)
    }
}
