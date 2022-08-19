use super::{Cell, Key, Player, PlayerType, Ship};
use crate::util;

pub struct Game {
    human: Player,
    computer: Player,
    board_size: usize,
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        Self {
            human: Player::new(board_size, PlayerType::Human),
            computer: Player::new(board_size, PlayerType::Computer),
            board_size,
        }
    }

    pub fn play(&mut self) {
        self.computer.place_ships_randomly();

        loop {
            util::clear_console();
            println!("SHIP SETUP");
            println!("1) Manually");
            println!("2) Randomly");

            match util::input_number() {
                1 => {
                    self.human.place_ships_manually();
                    break;
                }
                2 => {
                    self.human.place_ships_randomly();
                    break;
                }
                _ => {}
            }
        }

        let winner = loop {
            if self.human_turn() {
                break "You";
            }
            util::pause_console();

            if self.computer_turn() {
                break "Computer";
            }
            util::pause_console();
        };

        println!("{} won!", winner);
        util::pause_console();
    }

    pub fn print_boards(&mut self) {
        util::clear_console();
        println!("Computer's board:");
        self.computer.print_board(true);
        println!("Your board:");
        self.human.print_board(false);
    }

    pub fn human_turn(&mut self) -> bool {
        loop {
            self.print_boards();
            println!("YOUR TURN");
            println!("Choose your target");

            let mut x = self.board_size / 2;
            let mut y = self.board_size / 2;

            loop {
                util::move_cursor(x * 2 + 2, y + 2);
                match util::read_key() {
                    Key::ArrowLeft => {
                        if x > 0 {
                            x -= 1;
                        }
                    }
                    Key::ArrowRight => {
                        if x < self.board_size - 1 {
                            x += 1;
                        }
                    }
                    Key::ArrowUp => {
                        if y > 0 {
                            y -= 1;
                        }
                    }
                    Key::ArrowDown => {
                        if y < self.board_size - 1 {
                            y += 1;
                        }
                    }
                    Key::Enter => break,
                    _ => {}
                }
            }

            match self.try_turn(self.human.player_type, x, y) {
                Ok(_) => break,
                Err(_) => println!("Invalid location at {}, {}!", x, y),
            }
        }

        self.computer.all_ships_sunk()
    }

    pub fn computer_turn(&mut self) -> bool {
        self.print_boards();
        println!("COMPUTER'S TURN");

        loop {
            let x = util::rand_range(0, self.board_size);
            let y = util::rand_range(0, self.board_size);
            if self.try_turn(self.computer.player_type, x, y).is_ok() {
                break;
            }
        }

        self.human.all_ships_sunk()
    }

    pub fn try_turn(&mut self, player_type: PlayerType, x: usize, y: usize) -> Result<(), ()> {
        let opponent = match player_type {
            PlayerType::Human => &mut self.computer,
            PlayerType::Computer => &mut self.human,
        };

        match opponent.board[y][x] {
            Cell::Hit | Cell::Miss => return Err(()),
            Cell::Empty => {
                opponent.board[y][x] = Cell::Miss;
                self.print_boards();
                Game::miss_message(player_type, x, y);
            }
            _ => {
                for ship in opponent.ships.iter_mut() {
                    if opponent.board[y][x] == ship.cell {
                        opponent.board[y][x] = Cell::Hit;
                        ship.hits += 1;
                        Game::hit_message(*ship, player_type, x, y);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn miss_message(player_type: PlayerType, x: usize, y: usize) {
        match player_type {
            PlayerType::Human => println!("You missed at {}, {}!", x, y),
            PlayerType::Computer => println!("Computer missed at {}, {}!", x, y),
        }
    }

    pub fn hit_message(ship: Ship, player_type: PlayerType, x: usize, y: usize) {
        let verb = if ship.sunk() { "SUNK" } else { "hit" };

        match player_type {
            PlayerType::Human => println!("You {} computer's {} at {}, {}!", verb, ship.name, x, y),
            PlayerType::Computer => {
                println!("Computer {} your {} at {}, {}!", verb, ship.name, x, y)
            }
        }
    }
}
