use rand::distributions::{Distribution, Standard};

pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn rotate(&self, rotate: Rotation) -> Self {
        match rotate {
            Rotation::Clockwise => match self {
                Self::North => Self::NorthEast,
                Self::NorthEast => Self::SouthEast,
                Self::SouthEast => Self::South,
                Self::South => Self::SouthWest,
                Self::SouthWest => Self::NorthWest,
                Self::NorthWest => Self::North,
            },
            Rotation::CounterClockwise => match self {
                Self::North => Self::NorthWest,
                Self::NorthEast => Self::North,
                Self::SouthEast => Self::NorthEast,
                Self::South => Self::SouthEast,
                Self::SouthWest => Self::South,
                Self::NorthWest => Self::SouthWest,
            },
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=5) {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::NorthWest,
            3 => Direction::South,
            4 => Direction::SouthEast,
            _ => Direction::SouthWest,
        }
    }
}
