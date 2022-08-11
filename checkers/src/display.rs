use crate::{is_playable_spot, style, Board, Cell, Movable};

pub enum PrintType<'a> {
    Default,
    Movables(&'a Vec<Movable>),
    Moves(&'a Movable),
}

pub fn print_default(cell: &Cell) {
    if is_playable_spot(cell.pos.x, cell.pos.y) && cell.piece.is_none() {
        print!(" {}", style("-").dim());
    } else {
        print!(" {}", cell);
    }
}

pub fn print_movables(movables: &[Movable], cell: &Cell) {
    let mut movable_index = None;
    for (i, movable) in movables.iter().enumerate() {
        if movable.cell.pos == cell.pos {
            movable_index = Some(i);
            break;
        }
    }
    match movable_index {
        None => print_default(cell),
        Some(_) => print!(" {}", style(cell).bold().blue()),
    }
}

pub fn print_movable(movable: &Movable, cell: &Cell) {
    let mut move_index = None;
    for (i, move_) in movable.moves.iter().enumerate() {
        if move_.to.pos == cell.pos {
            move_index = Some(i);
        }
    }
    match move_index {
        None => {
            if movable.cell.pos == cell.pos {
                print!(" {}", style(cell).black().on_white());
            } else if movable.moves.iter().any(|m| match m.capture {
                Some(c) => c.pos == cell.pos,
                None => false,
            }) {
                print!(" {}", style(cell).on_red());
            } else {
                print_default(cell);
            }
        }
        Some(_) => print!(" {}", style('-').bold().blue()),
    }
}

pub fn print_board(board: &Board, print_type: PrintType) {
    for row in board {
        for cell in row {
            match print_type {
                PrintType::Default => print_default(cell),
                PrintType::Movables(movables) => print_movables(movables, cell),
                PrintType::Moves(movable) => print_movable(movable, cell),
            }
        }
        println!();
    }
    println!();
}
