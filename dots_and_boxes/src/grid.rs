use crate::{Player, Pos, Side, Square};

pub type Grid = Vec<Vec<Square>>;

pub trait GridExt {
    fn new_grid(size: usize) -> Self;
    fn checked_add(&self, sum: usize) -> Option<usize>;
    fn at(&mut self, pos: Pos) -> &mut Square;
    fn fill_side(&mut self, pos: Pos, side: Side, player: Player);
    fn get_adjacent(&self, pos: Pos, side: Side) -> Option<Pos>;
    fn is_filled(&self) -> bool;
    fn to_string(&self, selected_pos: Pos) -> String;
}

impl GridExt for Grid {
    fn new_grid(size: usize) -> Self {
        let mut grid = Vec::new();
        for y in 0..size {
            let mut row = Vec::new();
            for x in 0..size {
                row.push(Square::new(Pos::new(x, y)));
            }
            grid.push(row);
        }
        grid
    }

    fn checked_add(&self, sum: usize) -> Option<usize> {
        if sum < self.len() {
            Some(sum)
        } else {
            None
        }
    }

    fn at(&mut self, pos: Pos) -> &mut Square {
        &mut self[pos.y][pos.x]
    }

    fn fill_side(&mut self, pos: Pos, side: Side, player: Player) {
        self.at(pos).fill_side(side, player);
        if let Some(pos) = self.get_adjacent(pos, side) {
            self.at(pos).fill_side(side.get_opposite(), player);
        }
    }

    fn get_adjacent(&self, pos: Pos, side: Side) -> Option<Pos> {
        Some(match side {
            Side::Top => Pos::new(
                pos.x,
                match pos.y.checked_sub(1) {
                    None => return None,
                    Some(y) => y,
                },
            ),
            Side::Bottom => Pos::new(
                pos.x,
                match self.checked_add(pos.y + 1) {
                    None => return None,
                    Some(y) => y,
                },
            ),
            Side::Left => Pos::new(
                match pos.x.checked_sub(1) {
                    None => return None,
                    Some(x) => x,
                },
                pos.y,
            ),
            Side::Right => Pos::new(
                match self.checked_add(pos.x + 1) {
                    None => return None,
                    Some(x) => x,
                },
                pos.y,
            ),
        })
    }

    fn is_filled(&self) -> bool {
        self.iter().all(|row| row.iter().all(|square| square.is_filled()))
    }

    fn to_string(&self, selected_pos: Pos) -> String {
        let mut output = "\n ".to_string();
        for (x, square) in self[0].iter().enumerate() {
            output += square.side_row_to_string(x, square.top).as_str();
        }
        output += "\n ";
        for row in self {
            for (x, square) in row.iter().enumerate() {
                let is_selected_pos = square.pos == selected_pos;
                output += square.middle_to_string(x, is_selected_pos).as_str();
            }
            output += "\n ";
            for (x, square) in row.iter().enumerate() {
                output += square.side_row_to_string(x, square.bottom).as_str();
            }
            output += "\n ";
        }
        output
    }
}
