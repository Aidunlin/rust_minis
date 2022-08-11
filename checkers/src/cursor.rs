use crate::{Movable, Move, Pos};

pub fn get_to(from: usize, by: isize, num_choices: usize) -> usize {
    let to = from as isize + by + num_choices as isize;
    (to % num_choices as isize) as usize
}

pub fn move_cursor(pos: Pos<usize>) {
    console::Term::stdout()
        .move_cursor_to(pos.x * 2 + 1, pos.y)
        .expect("Failed to move cursor");
}

pub fn movables_move_cursor(movables: &Vec<Movable>, from: usize, by: isize) -> usize {
    let to = get_to(from, by, movables.len());
    move_cursor(movables[to].cell.pos);
    to
}

pub fn moves_move_cursor(moves: &Vec<Move>, from: usize, by: isize) -> usize {
    let to = get_to(from, by, moves.len());
    move_cursor(moves[to].to.pos);
    to
}
