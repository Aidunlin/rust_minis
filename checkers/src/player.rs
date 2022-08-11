use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Self::White => Self::Black,
            _ => Self::White,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::White => 'L',
            Self::Black => 'D',
        })
    }
}
