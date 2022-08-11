use console::Style;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn to_char(self, is_lower: bool) -> char {
        let c = match self {
            Self::A => 'A',
            Self::B => 'B',
        };
        if is_lower {
            c.to_ascii_lowercase()
        } else {
            c
        }
    }

    pub fn get_style(self) -> Style {
        match self {
            Player::A => Style::new().cyan(),
            Player::B => Style::new().red(),
        }
    }

    pub fn next(&mut self) {
        *self = match self {
            Self::A => Self::B,
            Self::B => Self::A,
        };
    }
}
