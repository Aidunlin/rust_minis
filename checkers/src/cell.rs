use crate::{Piece, Pos};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Cell {
    pub piece: Option<Piece>,
    pub pos: Pos<usize>,
}

impl Cell {
    pub fn new(piece: Option<Piece>, pos: Pos<usize>) -> Self {
        Self { piece, pos }
    }

    pub fn convert_piece(self, row: usize, board_size: usize) -> Option<Piece> {
        match self.piece {
            None => self,
            Some(piece) => Self {
                piece: Some(piece.convert(row, board_size)),
                ..self
            },
        }
        .piece
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.piece {
            None => write!(f, " "),
            Some(piece) => write!(f, "{}", piece),
        }
    }
}
