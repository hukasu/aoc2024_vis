use std::collections::BTreeMap;

use bevy::{color::Color, prelude::Resource};

use crate::loader::RawInput;

pub type Coord = (usize, usize);

#[derive(Debug, Resource)]
pub struct Input {
    pub bounds: Coord,
    pub antennas: BTreeMap<Coord, Color>,
    pub slopes: Vec<Slope>,
}

impl Input {
    pub fn parse(input: &RawInput) -> Self {
        let input = input
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let bounds = (input[0].len(), input.len());

        let mut base_color = Color::srgb_u8(0, 0, 0);

        let mut colors = BTreeMap::new();

        let mut antennas = BTreeMap::new();
        let mut slopes = vec![];

        let mut signal = BTreeMap::new();

        for (y, line) in input.into_iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != b'.' {
                    let color: Color = *colors.entry(c).or_insert_with(|| {
                        let color = base_color.to_srgba();
                        base_color = base_color
                            .to_srgba()
                            .with_red((color.red + 7. / 255.).fract())
                            .with_green((color.green + 13. / 255.).fract())
                            .with_blue((color.blue + 58. / 255.).fract())
                            .into();
                        color.into()
                    });
                    antennas.insert((x, y), color);

                    let similar_antennas = signal.entry(c).or_insert_with(Vec::<Coord>::new);

                    for other_antenna in similar_antennas.iter() {
                        let (top, bottom) = if other_antenna.1 < y {
                            (*other_antenna, (x, y))
                        } else {
                            ((x, y), *other_antenna)
                        };

                        slopes.push(Slope {
                            zero: top,
                            offset: (
                                isize::try_from(bottom.0).unwrap()
                                    - isize::try_from(top.0).unwrap(),
                                isize::try_from(bottom.1).unwrap()
                                    - isize::try_from(top.1).unwrap(),
                            ),
                            bounds,
                            color,
                        });
                    }

                    similar_antennas.push((x, y));
                }
            }
        }

        Self {
            bounds,
            antennas,
            slopes,
        }
    }
}

#[derive(Debug)]
pub struct Slope {
    zero: Coord,
    offset: (isize, isize),
    bounds: Coord,
    pub color: Color,
}

impl Slope {
    pub fn interpolate(&self, t: isize) -> Option<Coord> {
        let x = self
            .zero
            .0
            .checked_add_signed(self.offset.0 * t)
            .filter(|x| *x < self.bounds.0)?;
        let y = self
            .zero
            .1
            .checked_add_signed(self.offset.1 * t)
            .filter(|y| *y < self.bounds.1)?;

        Some((x, y))
    }
}
