use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: usize,
    pub column: usize,
}

impl Coord {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl Add<(usize, usize)> for Coord {
    type Output = Self;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            column: self.column + rhs.1,
        }
    }
}

impl Sub<(usize, usize)> for Coord {
    type Output = Self;

    fn sub(self, rhs: (usize, usize)) -> Self::Output {
        Self {
            row: self.row - rhs.0,
            column: self.column - rhs.1,
        }
    }
}
