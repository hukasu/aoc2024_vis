use std::{borrow::Borrow, cmp::Ordering};

use bevy::prelude::Resource;

use crate::loader::RawInput;

pub const BOUNDS: (i64, i64) = (101, 103);

#[derive(Debug, Resource)]
pub struct Input {
    pub start: Vec<Robot>,
    pub robots: Vec<Robot>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input.split(|c| *c == b'\n').filter(|line| !line.is_empty());

        let robots = input
            .map(|line| {
                let split1 = line.split(|c| *c == b' ').collect::<Vec<_>>();
                let [p, v] = split1.as_slice() else {
                    unreachable!("Will always have 2 items.")
                };

                let splitp = p[2..]
                    .split(|c| *c == b',')
                    .map(|val| String::from_utf8_lossy(val).parse())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let splitv = v[2..]
                    .split(|c| *c == b',')
                    .map(|val| String::from_utf8_lossy(val).parse())
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                Robot {
                    position: (splitp[0], splitp[1]),
                    velocity: (splitv[0], splitv[1]),
                }
            })
            .collect::<Vec<_>>();

        Self {
            start: robots.clone(),
            robots,
        }
    }

    pub fn step(&mut self, seconds: i64) {
        self.robots
            .iter_mut()
            .for_each(|robot| robot.step(seconds, BOUNDS));
    }

    pub fn safety_factor(&self, seconds: i64) -> isize {
        let quadrants = self
            .start
            .iter()
            .map(|robot| {
                let mut robot = *robot;
                robot.step(seconds, BOUNDS);
                robot
            })
            .fold((0, 0, 0, 0), |a, b| {
                Self::count_robots_in_quadrant(a, b, BOUNDS)
            });

        quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
    }

    pub fn easter_egg(&self) -> i64 {
        let mut robots = self.start.clone();
        (1..=(BOUNDS.0 * BOUNDS.1))
            .map(|i| {
                robots.iter_mut().for_each(|robot| robot.step(1, BOUNDS));
                let quadrants = robots.iter().fold((0, 0, 0, 0), |a, b| {
                    Self::count_robots_in_quadrant(a, b, BOUNDS)
                });
                (
                    i,
                    quadrants
                        .0
                        .max(quadrants.1)
                        .max(quadrants.2)
                        .max(quadrants.3),
                )
            })
            .max_by_key(|(_, score)| *score)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn count_robots_in_quadrant(
        quadrants: (isize, isize, isize, isize),
        robot: impl Borrow<Robot>,
        bounds: (i64, i64),
    ) -> (isize, isize, isize, isize) {
        match (
            robot.borrow().position.0.cmp(&(bounds.0 / 2)),
            robot.borrow().position.1.cmp(&(bounds.1 / 2)),
        ) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => quadrants,
            (Ordering::Less, Ordering::Less) => {
                (quadrants.0 + 1, quadrants.1, quadrants.2, quadrants.3)
            }
            (Ordering::Less, Ordering::Greater) => {
                (quadrants.0, quadrants.1 + 1, quadrants.2, quadrants.3)
            }
            (Ordering::Greater, Ordering::Less) => {
                (quadrants.0, quadrants.1, quadrants.2 + 1, quadrants.3)
            }
            (Ordering::Greater, Ordering::Greater) => {
                (quadrants.0, quadrants.1, quadrants.2, quadrants.3 + 1)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pub position: (i64, i64),
    pub velocity: (i64, i64),
}

impl Robot {
    pub fn step(&mut self, steps: i64, bounds: (i64, i64)) {
        self.position.0 += self.velocity.0 * steps;
        self.position.0 = self.position.0.rem_euclid(bounds.0);
        self.position.1 += self.velocity.1 * steps;
        self.position.1 = self.position.1.rem_euclid(bounds.1);
    }
}
