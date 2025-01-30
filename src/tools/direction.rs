use super::Coord;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    pub fn step_unchecked(&self, coord: Coord) -> Coord {
        match self {
            Direction::North => coord - (1, 0),
            Direction::South => coord + (1, 0),
            Direction::East => coord + (0, 1),
            Direction::West => coord - (0, 1),
        }
    }

    pub fn step(&self, coord: Coord, bounds: Coord) -> Option<Coord> {
        match self {
            Direction::North => coord
                .row
                .checked_sub(1)
                .map(|y| Coord::new(y, coord.column)),
            Direction::South => Some(coord + (1, 0)).filter(|coord| coord.row < bounds.row),
            Direction::East => Some(coord + (0, 1)).filter(|coord| coord.column < bounds.column),
            Direction::West => coord
                .column
                .checked_sub(1)
                .map(|x| Coord::new(coord.row, x)),
        }
    }
}
