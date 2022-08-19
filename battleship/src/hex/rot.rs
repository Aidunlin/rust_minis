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
    pub fn rotate(&self, rotate: Rotation) -> Direction {
        match rotate {
            Rotation::Clockwise => match self {
                Direction::North => Direction::NorthEast,
                Direction::NorthEast => Direction::SouthEast,
                Direction::SouthEast => Direction::South,
                Direction::South => Direction::SouthWest,
                Direction::SouthWest => Direction::NorthWest,
                Direction::NorthWest => Direction::North,
            },
            Rotation::CounterClockwise => match self {
                Direction::North => Direction::NorthWest,
                Direction::NorthEast => Direction::North,
                Direction::SouthEast => Direction::NorthEast,
                Direction::South => Direction::SouthEast,
                Direction::SouthWest => Direction::South,
                Direction::NorthWest => Direction::SouthWest,
            },
        }
    }
}
