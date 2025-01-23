use bevy::prelude::Resource;

use crate::loader::RawInput;

type Coord = (i64, i64);

#[derive(Debug, Resource)]
pub struct Input {
    pub machines: Vec<ClawMachine>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let mut input = input.split(|c| *c == b'\n');

        let mut machines = vec![];

        loop {
            let Some(a) = input.next() else {
                break;
            };
            let a = {
                let split = a.split(|c| *c == b' ').collect::<Vec<_>>();
                let [b"Button", b"A:", x, y] = split.as_slice() else {
                    unreachable!("AoC inputs are well formed.");
                };
                (
                    String::from_utf8_lossy(&x[2..(x.len() - 1)])
                        .parse()
                        .unwrap(),
                    String::from_utf8_lossy(&y[2..]).parse().unwrap(),
                )
            };

            let b = input.next().unwrap();
            let b = {
                let split = b.split(|c| *c == b' ').collect::<Vec<_>>();
                let [b"Button", b"B:", x, y] = split.as_slice() else {
                    unreachable!("AoC inputs are well formed.");
                };
                (
                    String::from_utf8_lossy(&x[2..(x.len() - 1)])
                        .parse()
                        .unwrap(),
                    String::from_utf8_lossy(&y[2..]).parse().unwrap(),
                )
            };

            let reward = input.next().unwrap();
            let reward = {
                let split = reward.split(|c| *c == b' ').collect::<Vec<_>>();
                let [b"Prize:", x, y] = split.as_slice() else {
                    unreachable!("AoC inputs are well formed.");
                };
                (
                    String::from_utf8_lossy(&x[2..(x.len() - 1)])
                        .parse()
                        .unwrap(),
                    String::from_utf8_lossy(&y[2..]).parse().unwrap(),
                )
            };

            let machine_break = input.next().unwrap();
            assert!(machine_break.is_empty());

            machines.push(ClawMachine {
                button_a: a,
                button_b: b,
                prize: reward,
            });
        }

        Self { machines }
    }
}

#[derive(Debug)]
pub struct ClawMachine {
    pub button_a: Coord,
    pub button_b: Coord,
    pub prize: Coord,
}

impl ClawMachine {
    pub fn find_cheapest_solution(&self, padding: i64) -> Option<(i64, i64, i64)> {
        let b = ((self.prize.0 + padding) * self.button_a.1
            - (self.prize.1 + padding) * self.button_a.0)
            / (self.button_b.0 * self.button_a.1 - self.button_a.0 * self.button_b.1);
        let a = ((self.prize.1 + padding) - b * self.button_b.1) / self.button_a.1;
        if (self.prize.0 + padding) == a * self.button_a.0 + b * self.button_b.0
            && (self.prize.1 + padding) == a * self.button_a.1 + b * self.button_b.1
            && (0..10000000000000).contains(&a)
            && (0..10000000000000).contains(&b)
        {
            Some((a * 3 + b, a, b))
        } else {
            None
        }
    }
}
