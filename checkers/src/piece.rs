use crate::Player;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Piece {
    pub player: Player,
    pub king: bool,
}

impl Piece {
    pub fn new(player: Player) -> Option<Self> {
        Some(Self {
            player,
            king: false,
        })
    }

    pub fn convert(self, row: usize, board_size: usize) -> Self {
        match self.player {
            Player::White if row >= board_size - 1 => Self { king: true, ..self },
            Player::Black if row == 0 => Self { king: true, ..self },
            _ => self,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!("{}", self.player);
        if self.king {
            write!(f, "{}", output)
        } else {
            write!(f, "{}", output.to_ascii_lowercase())
        }
    }
}
