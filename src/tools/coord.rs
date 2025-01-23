use std::ops::{Add, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: usize,
    pub column: usize,
}

impl Coord {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn dist(&self, other: &Coord) -> usize {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }

    pub fn adjacent_4_way(&self, bounds: Coord) -> [Option<Coord>; 4] {
        [
            self.row
                .checked_sub(1)
                .map(|row| Coord::new(row, self.column)),
            self.row
                .checked_add(1)
                .filter(|row| *row < bounds.row)
                .map(|row| Coord::new(row, self.column)),
            self.column
                .checked_sub(1)
                .map(|column| Coord::new(self.row, column)),
            self.column
                .checked_add(1)
                .filter(|column| *column < bounds.column)
                .map(|column| Coord::new(self.row, column)),
        ]
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
