use crate::{util, Cell, Ship};

pub enum Axis {
    Horizontal,
    Vertical,
}

pub enum PlayerType {
    Human,
    Computer,
}

pub struct Player {
    pub player_type: PlayerType,
    pub board: Vec<Vec<Cell>>,
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn new(board_size: usize, player_type: PlayerType) -> Self {
        let mut ships = Vec::new();
        ships.push(Ship::new('C', "Carrier", 5));
        ships.push(Ship::new('B', "Battleship", 4));
        ships.push(Ship::new('R', "Cruiser", 3));
        ships.push(Ship::new('S', "Submarine", 3));
        ships.push(Ship::new('D', "Destroyer", 2));

        Self {
            player_type,
            board: vec![vec![Cell::Empty; board_size]; board_size],
            ships,
        }
    }

    pub fn print_board(&self, hide_ships: bool) {
        print!(" ");

        for i in 0..self.board.len() {
            print!(" {}", i);
        }

        println!();

        for (i, row) in self.board.iter().enumerate() {
            print!("{}", i);

            for col in row {
                let cell = match col {
                    Cell::Empty => '-',
                    Cell::Hit => '*',
                    Cell::Miss => 'm',
                    Cell::Ship(_) if hide_ships => '-',
                    Cell::Ship(ship) => *ship,
                };

                print!(" {}", cell);
            }

            println!();
        }
    }

    pub fn place_ships_manually(&mut self) {
        for i in 0..self.ships.len() {
            loop {
                console::Term::stdout().clear_screen().unwrap();
                println!("SHIP SETUP");
                println!("{} ({} cells)", self.ships[i].name, self.ships[i].size);
                self.print_board(false);

                println!("Enter an axis (0 for horizontal, 1 for vertical),");
                println!("then an x and y value that represents the");
                println!("top-most or left-most cell of the ship");

                let axis = util::input_number::<usize>();
                let mut x = util::input_number::<usize>();
                let mut y = util::input_number::<usize>();

                let axis = match axis {
                    0 => {
                        x = x.clamp(0, self.board.len() - self.ships[i].size);
                        y = y.clamp(0, self.board.len() - 1);
                        Axis::Horizontal
                    }
                    1 => {
                        x = x.clamp(0, self.board.len() - 1);
                        y = y.clamp(0, self.board.len() - self.ships[i].size);
                        Axis::Vertical
                    }
                    _ => continue,
                };

                if self.place_ship(i, axis, x, y) {
                    break;
                } else {
                    println!("Invalid location at {}, {}!", x, y);
                    util::pause_console();
                }
            }
        }
    }

    pub fn place_ships_randomly(&mut self) {
        for i in 0..self.ships.len() {
            loop {
                let larger_axis = util::rand_range(0, self.board.len());
                let smaller_axis = util::rand_range(0, self.board.len() - self.ships[i].size);

                let x: usize;
                let y: usize;
                let axis = match util::rand_range(0, 2) {
                    0 => {
                        x = smaller_axis;
                        y = larger_axis;
                        Axis::Horizontal
                    }
                    _ => {
                        x = larger_axis;
                        y = smaller_axis;
                        Axis::Vertical
                    }
                };

                if self.place_ship(i, axis, x, y) {
                    break;
                }
            }
        }
    }

    pub fn place_ship(&mut self, ship: usize, axis: Axis, x: usize, y: usize) -> bool {
        let mut valid = true;

        for i in 0..self.ships[ship].size {
            if match axis {
                Axis::Horizontal => self.board[y][x + i] != Cell::Empty,
                Axis::Vertical => self.board[y + i][x] != Cell::Empty,
            } {
                valid = false;
                break;
            }
        }

        if valid {
            for i in 0..self.ships[ship].size {
                match axis {
                    Axis::Horizontal => self.board[y][x + i] = self.ships[ship].cell,
                    Axis::Vertical => self.board[y + i][x] = self.ships[ship].cell,
                }
            }
        }

        valid
    }

    pub fn all_ships_sunk(&mut self) -> bool {
        for ship in &self.ships {
            if !ship.sunk {
                return false;
            }
        }

        return true;
    }
}
