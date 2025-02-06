use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::{Component, Resource};

use crate::loader::RawInput;

#[derive(Debug, Clone, Resource, Component)]
pub struct Input {
    pub rng_seeds: Vec<usize>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let rng_seeds = input
            .map(|line| String::from_utf8_lossy(line).parse().unwrap())
            .collect();

        Self { rng_seeds }
    }

    pub fn rngs(&self) -> Vec<Rng> {
        self.rng_seeds
            .iter()
            .map(|seed| Rng { secret: *seed })
            .collect()
    }

    pub fn part2(&self) -> ([isize; 4], usize) {
        let time_series = self.time_series(2000);

        let unique_windows = BTreeSet::from_iter(
            time_series
                .iter()
                .flat_map(|time_series| time_series.price_changes_windows.keys()),
        );

        unique_windows
            .into_iter()
            .map(|windows| {
                (
                    *windows,
                    time_series
                        .iter()
                        .flat_map(|time_series| time_series.price_changes_windows.get(windows))
                        .sum(),
                )
            })
            .max_by_key(|(_, sum)| *sum)
            .unwrap()
    }

    fn time_series(&self, updates: usize) -> Vec<TimeSeries> {
        let mut time_series = vec![TimeSeries::default(); self.rng_seeds.len()];

        std::thread::scope(|scope| {
            let paralelism = std::thread::available_parallelism().unwrap().get();
            let input_len = self.rng_seeds.len();

            let input_chunks = self.rng_seeds.chunks((input_len / paralelism) + 1);
            let time_series_chunks = time_series.chunks_mut((input_len / paralelism) + 1);

            for (input_chunk, time_series_chunk) in input_chunks.zip(time_series_chunks) {
                scope.spawn(move || {
                    for (num, time_series) in input_chunk.iter().zip(time_series_chunk.iter_mut()) {
                        let mut rng = Rng { secret: *num };
                        let mut prev = None;
                        let mut windows = Vec::new();
                        for _ in 0..updates {
                            let price = rng.secret % 10;
                            time_series.price_time_series.push(price);
                            if let Some(prev) = prev {
                                let price_change = isize::try_from(price).unwrap()
                                    - isize::try_from(prev).unwrap();
                                time_series.price_changes.push(price_change);
                                windows.push(price_change);
                                if windows.len() > 4 {
                                    windows.remove(0);
                                }
                            }
                            if windows.len() == 4 {
                                let window = windows.as_slice().try_into().unwrap();
                                time_series
                                    .price_changes_windows
                                    .entry(window)
                                    .or_insert(price);
                            }

                            rng.process();
                            prev.replace(price);
                        }
                    }
                });
            }
        });

        time_series
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rng {
    pub secret: usize,
}

impl Rng {
    pub fn find_first_occurance(&self, series: &[isize; 4], limit: usize) -> Option<[isize; 5]> {
        let mut window = vec![self.secret as isize % 10];

        for price in (*self).map(|price| price as isize % 10).take(limit) {
            if window.len() == 5 {
                if window
                    .windows(2)
                    .map(|win| win[1] - win[0])
                    .collect::<Vec<_>>()
                    == series
                {
                    return Some(window.try_into().unwrap());
                }
                window.remove(0);
            }
            window.push(price);
        }

        None
    }

    fn process(&mut self) {
        let first_round = Self::prune(Self::mix(self.secret * 64, self.secret));
        let second_round = Self::prune(Self::mix(first_round / 32, first_round));
        self.secret = Self::prune(Self::mix(second_round * 2048, second_round))
    }

    fn mix(value: usize, secret: usize) -> usize {
        value ^ secret
    }

    fn prune(secret: usize) -> usize {
        secret % 16777216
    }
}

impl Iterator for Rng {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.process();
        Some(self.secret)
    }
}

#[derive(Debug, Default, Clone)]
struct TimeSeries {
    price_time_series: Vec<usize>,
    price_changes: Vec<isize>,
    price_changes_windows: BTreeMap<[isize; 4], usize>,
}
