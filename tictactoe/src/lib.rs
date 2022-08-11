use console::{Key, Term};

fn clear_console() {
    Term::stdout()
        .clear_screen()
        .expect("Failed to clear screen");
}

type Board = Vec<Vec<Option<char>>>;

const PLAYER_X: char = 'x';
const PLAYER_O: char = 'o';

enum GameResult {
    Win(char),
    Tie,
}

pub struct Tictactoe {
    board: Board,
    size: usize,
    player: char,
    turns: usize,
}

impl Tictactoe {
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![vec![None; size]; size],
            size,
            player: PLAYER_X,
            turns: 0,
        }
    }

    pub fn play(&mut self) {
        let result = loop {
            self.print_board();
            println!();
            println!("{}'s turn", self.player);
            println!("Choose a spot");
            let (row, column) = self.get_choice();
            if self.set_if_none(row, column) {
                if self.has_won(row, column) {
                    break GameResult::Win(self.player);
                }
                self.turns += 1;
                if self.has_tied() {
                    break GameResult::Tie;
                }
                self.player = self.next_player();
            }
        };
        let result_message = match result {
            GameResult::Win(player) => player.to_string() + " won!",
            GameResult::Tie => "Tie!".to_string(),
        };
        self.print_board();
        println!("{}", result_message);
    }

    fn get_choice(&self) -> (usize, usize) {
        let mut choice = self.move_cursor(0, 0);
        loop {
            match Term::stdout().read_key().expect("Failed to read key") {
                Key::ArrowLeft => choice = self.move_cursor(choice, -1),
                Key::ArrowRight => choice = self.move_cursor(choice, 1),
                Key::ArrowUp => choice = self.move_cursor(choice, -(self.size as isize)),
                Key::ArrowDown => choice = self.move_cursor(choice, self.size as isize),
                Key::Enter => break,
                Key::Escape => {
                    clear_console();
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        (choice / self.size, choice % self.size)
    }

    fn move_cursor(&self, from: usize, by: isize) -> usize {
        let to = from as isize + by + self.size.pow(2) as isize;
        let to = (to % self.size.pow(2) as isize) as usize;
        Term::stdout()
            .move_cursor_to(to % self.size * 2 + 1, to / self.size)
            .expect("Failed to move cursor");
        to
    }

    fn print_board(&self) {
        clear_console();
        for row in &self.board {
            for cell in row {
                match cell {
                    None => print!(" -"),
                    Some(player) => print!(" {}", player),
                };
            }
            println!();
        }
    }

    fn next_player(&self) -> char {
        match self.player {
            PLAYER_X => PLAYER_O,
            _ => PLAYER_X,
        }
    }

    fn set_if_none(&mut self, row: usize, column: usize) -> bool {
        if self.board[row][column].is_none() {
            self.board[row][column] = Some(self.player);
            return true;
        }
        false
    }

    fn cell_is_player(&self, row: usize, column: usize) -> bool {
        match self.board[row][column] {
            None => false,
            Some(p) => p == self.player,
        }
    }

    fn has_won(&self, row: usize, column: usize) -> bool {
        let mut won_row = true;
        let mut won_column = true;
        let mut won_positive = true;
        let mut won_negative = true;
        for i in 0..self.size {
            if !self.cell_is_player(row, i) {
                won_row = false;
            }
            if !self.cell_is_player(i, column) {
                won_column = false;
            }
            if !self.cell_is_player(i, i) {
                won_positive = false;
            }
            if !self.cell_is_player(self.size - 1 - i, i) {
                won_negative = false;
            }
        }
        won_row || won_column || won_positive || won_negative
    }

    fn has_tied(&self) -> bool {
        self.turns >= self.size.pow(2)
    }
}
