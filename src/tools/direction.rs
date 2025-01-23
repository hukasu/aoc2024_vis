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

    pub fn step(&self, coord: Coord) -> Coord {
        match self {
            Direction::North => coord - (1, 0),
            Direction::South => coord + (1, 0),
            Direction::East => coord + (0, 1),
            Direction::West => coord - (0, 1),
        }
    }
}
