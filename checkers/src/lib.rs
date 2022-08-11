mod cell;
mod cursor;
mod display;
mod piece;
mod player;
mod pos;

pub use cell::Cell;
pub use console::{style, Key, Term};
pub use display::*;
pub use piece::Piece;
pub use player::Player;
pub use pos::{Dir, Pos};

fn clear_console() {
    Term::stdout()
        .clear_screen()
        .expect("Failed to clear console");
}

fn exit() {
    clear_console();
    std::process::exit(0);
}

fn is_playable_spot(x: usize, y: usize) -> bool {
    (y + x) % 2 != 0
}

pub struct Move {
    pub to: Cell,
    pub capture: Option<Cell>,
}

impl Move {
    fn new(to: Cell, capture: Option<Cell>) -> Self {
        Move { to, capture }
    }
}

pub struct Movable {
    pub cell: Cell,
    pub moves: Vec<Move>,
}

type Board = Vec<Vec<Cell>>;

trait BoardExt {
    fn at(&self, pos: &Pos<usize>) -> Cell;
    fn set(&mut self, pos: Pos<usize>, piece: Option<Piece>) -> Cell;
}

impl BoardExt for Board {
    fn at(&self, pos: &Pos<usize>) -> Cell {
        self[pos.y][pos.x]
    }

    fn set(&mut self, pos: Pos<usize>, piece: Option<Piece>) -> Cell {
        self[pos.y][pos.x] = Cell::new(piece, pos);
        self.at(&pos)
    }
}

pub struct Checkers {
    pub board: Board,
    pub player: Player,
}

impl Checkers {
    pub fn new(size: usize) -> Self {
        let mut board = Vec::new();
        for y in 0..size {
            let mut row: Vec<Cell> = Vec::new();
            for x in 0..size {
                row.push(Cell::new(None, Pos { x, y }));
            }
            board.push(row);
        }
        let mut count = 0;
        for y in 0..size {
            for x in 0..size {
                if is_playable_spot(x, y) && count < 12 {
                    board.set(Pos { x, y }, Piece::new(Player::White));
                    board.set(
                        Pos {
                            x: size - 1 - x,
                            y: size - 1 - y,
                        },
                        Piece::new(Player::Black),
                    );
                    count += 1;
                }
            }
        }
        Checkers {
            board,
            player: Player::White,
        }
    }

    fn get_directions(&self, cell: &Cell) -> Vec<Dir> {
        match cell.piece {
            None => Vec::new(),
            Some(piece) => {
                if piece.king {
                    vec![Dir::NW, Dir::NE, Dir::SW, Dir::SE]
                } else {
                    match piece.player {
                        Player::White => vec![Dir::SW, Dir::SE],
                        Player::Black => vec![Dir::NW, Dir::NE],
                    }
                }
            }
        }
    }

    fn get_moves(&self, board: &Board, cell: Cell) -> Vec<Move> {
        let mut moves = Vec::new();
        if let Some(piece) = cell.piece {
            for direction in self.get_directions(&cell).iter() {
                let walk_pos = cell.pos.move_by(direction);
                if !walk_pos.is_valid(board.len()) {
                    continue;
                }
                let walk_cell = board.at(&walk_pos);
                let jump_pos = walk_pos.move_by(direction);
                match walk_cell.piece {
                    None => moves.push(Move::new(walk_cell, None)),
                    Some(p) if p.player != piece.player && jump_pos.is_valid(board.len()) => {
                        let jump_cell = board.at(&jump_pos);
                        if jump_cell.piece.is_none() {
                            moves.push(Move::new(jump_cell, Some(walk_cell)));
                        }
                    }
                    _ => {}
                }
            }
        }
        moves
    }

    fn get_movables(&self, checkers: &Checkers) -> Vec<Movable> {
        let mut movables = Vec::new();
        for row in &checkers.board {
            for cell in row {
                if let Some(piece) = cell.piece {
                    if piece.player != checkers.player {
                        continue;
                    }
                    let movable = Movable {
                        cell: *cell,
                        moves: self.get_moves(&checkers.board, *cell),
                    };
                    if !movable.moves.is_empty() {
                        movables.push(movable);
                    }
                }
            }
        }
        movables
    }

    fn do_move(&mut self, movable: &Movable, move_choice: usize) -> (Cell, bool) {
        let move_ = &movable.moves[move_choice];
        let piece = movable.cell.convert_piece(move_.to.pos.y, self.board.len());
        self.board.set(movable.cell.pos, None);
        let captured = if let Some(capture) = move_.capture {
            self.board.set(capture.pos, None);
            true
        } else {
            false
        };
        (self.board.set(move_.to.pos, piece), captured)
    }

    fn inner_turn_loop(&mut self, player: Player, movable: &Movable) -> bool {
        clear_console();
        print_board(&self.board, PrintType::Moves(movable));
        println!("{}'s turn", player);
        println!("{}", style("Choose a move").bold().blue());
        let mut choice = cursor::moves_move_cursor(&movable.moves, 0, 0);
        loop {
            let key = console::Term::stdout()
                .read_key()
                .expect("Failed to read key");
            match key {
                Key::ArrowLeft => choice = cursor::moves_move_cursor(&movable.moves, choice, -1),
                Key::ArrowRight => choice = cursor::moves_move_cursor(&movable.moves, choice, 1),
                Key::Enter => break,
                Key::Escape => exit(),
                _ => {}
            }
        }
        let (new_cell, captured) = self.do_move(movable, choice);
        let new_movable = Movable {
            cell: new_cell,
            moves: self
                .get_moves(&self.board, new_cell)
                .into_iter()
                .filter(|m| m.capture.is_some())
                .collect(),
        };
        if captured && !new_movable.moves.is_empty() {
            self.inner_turn_loop(player, &new_movable)
        } else {
            true
        }
    }

    pub fn play(&mut self) {
        let winner = loop {
            let movables = self.get_movables(self);
            if movables.is_empty() {
                break self.player.next();
            }
            clear_console();
            print_board(&self.board, PrintType::Movables(&movables));
            println!("{}'s turn", self.player);
            println!("{}", style("Choose a piece").bold().blue());
            let mut choice = cursor::movables_move_cursor(&movables, 0, 0);
            loop {
                let key = console::Term::stdout()
                    .read_key()
                    .expect("Failed to read key");
                match key {
                    Key::ArrowLeft => choice = cursor::movables_move_cursor(&movables, choice, -1),
                    Key::ArrowRight => choice = cursor::movables_move_cursor(&movables, choice, 1),
                    Key::Enter => break,
                    Key::Escape => exit(),
                    _ => {}
                }
            }
            loop {
                if self.inner_turn_loop(self.player, &movables[choice]) {
                    break;
                }
            }
            self.player = self.player.next();
        };
        clear_console();
        print_board(&self.board, PrintType::Default);
        println!("{} won!", style(winner).bold());
    }
}
