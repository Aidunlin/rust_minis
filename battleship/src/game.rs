use crate::{util, Cell, Player, PlayerType};

pub struct Game {
    human: Player,
    computer: Player,
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        let mut game = Self {
            human: Player::new(board_size, PlayerType::Human),
            computer: Player::new(board_size, PlayerType::Computer),
        };

        game.computer.place_ships_randomly();

        loop {
            util::clear_console();
            println!("SHIP SETUP");
            println!("1) Manually");
            println!("2) Randomly");

            match util::input_number() {
                1 => {
                    game.human.place_ships_manually();
                    break;
                }
                2 => {
                    game.human.place_ships_randomly();
                    break;
                }
                _ => {}
            }
        }

        game
    }

    pub fn play(&mut self) {
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

            println!("Enter an x and y value that represents your target:");
            let x = util::input_number::<usize>().clamp(0, self.human.board.len() - 1);
            let y = util::input_number::<usize>().clamp(0, self.human.board.len() - 1);

            if self.handle_turn(PlayerType::Human, x, y) {
                break;
            } else {
                println!("Invalid location at {}, {}!", x, y);
            }
        }

        self.computer.all_ships_sunk()
    }

    pub fn computer_turn(&mut self) -> bool {
        self.print_boards();
        println!("COMPUTER'S TURN");

        loop {
            let x = util::rand_range(0, self.computer.board.len());
            let y = util::rand_range(0, self.computer.board.len());
            if self.handle_turn(PlayerType::Computer, x, y) {
                break;
            }
        }

        self.human.all_ships_sunk()
    }

    pub fn handle_turn(&mut self, player: PlayerType, x: usize, y: usize) -> bool {
        let mut found_place = true;

        let opponent = match player {
            PlayerType::Human => &mut self.computer,
            PlayerType::Computer => &mut self.human,
        };

        if opponent.board[y][x] == Cell::Empty {
            opponent.board[y][x] = Cell::Miss;

            match player {
                PlayerType::Human => println!("You missed at {}, {}!", x, y),
                PlayerType::Computer => println!("Computer missed at {}, {}!", x, y),
            }
        } else if opponent.board[y][x] == Cell::Hit || opponent.board[y][x] == Cell::Miss {
            found_place = false;
        } else {
            for i in 0..opponent.ships.len() {
                if opponent.board[y][x] == opponent.ships[i].cell {
                    opponent.board[y][x] = Cell::Hit;
                    opponent.ships[i].hits += 1;

                    let verb = match opponent.ships[i].hits >= opponent.ships[i].size {
                        true => {
                            opponent.ships[i].sunk = true;
                            "SUNK"
                        }
                        false => "hit",
                    };

                    match player {
                        PlayerType::Human => println!(
                            "You {} computer's {} at {}, {}!",
                            verb, opponent.ships[i].name, x, y
                        ),
                        PlayerType::Computer => println!(
                            "Computer {} your {} at {}, {}!",
                            verb, opponent.ships[i].name, x, y
                        ),
                    }
                }
            }
        }

        found_place
    }
}
