use super::{Cell, Ship};
use crate::util;

pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
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
        for ship in self.ships.clone() {
            loop {
                util::clear_console();
                println!("SHIP SETUP");
                println!("{} ({} cells)", ship.name, ship.size);
                self.print_board(false);

                println!("Enter an axis (0 for horizontal, 1 for vertical),");
                println!("then an x and y value that represents the");
                println!("top-most or left-most cell of the ship");

                let axis = util::input_number::<usize>();
                let mut x = util::input_number::<usize>();
                let mut y = util::input_number::<usize>();

                let axis = match axis {
                    0 => {
                        x = x.clamp(0, self.board.len() - ship.size);
                        y = y.clamp(0, self.board.len() - 1);
                        Axis::Horizontal
                    }
                    1 => {
                        x = x.clamp(0, self.board.len() - 1);
                        y = y.clamp(0, self.board.len() - ship.size);
                        Axis::Vertical
                    }
                    _ => continue,
                };

                match self.try_place_ship(ship, axis, x, y) {
                    Ok(_) => break,
                    Err(_) => {
                        println!("Invalid location at {}, {}!", x, y);
                        util::pause_console();
                    }
                }
            }
        }
    }

    pub fn place_ships_randomly(&mut self) {
        for ship in self.ships.clone() {
            loop {
                let larger_axis = util::rand_range(0, self.board.len());
                let smaller_axis = util::rand_range(0, self.board.len() - ship.size);

                let x: usize;
                let y: usize;
                let axis = match rand::random() {
                    true => {
                        x = smaller_axis;
                        y = larger_axis;
                        Axis::Horizontal
                    }
                    false => {
                        x = larger_axis;
                        y = smaller_axis;
                        Axis::Vertical
                    }
                };

                if self.try_place_ship(ship, axis, x, y).is_ok() {
                    break;
                }
            }
        }
    }

    pub fn try_place_ship(&mut self, ship: Ship, axis: Axis, x: usize, y: usize) -> Result<(), ()> {
        for i in 0..ship.size {
            if match axis {
                Axis::Horizontal => self.board[y][x + i] != Cell::Empty,
                Axis::Vertical => self.board[y + i][x] != Cell::Empty,
            } {
                return Err(());
            }
        }

        for i in 0..ship.size {
            match axis {
                Axis::Horizontal => self.board[y][x + i] = ship.cell,
                Axis::Vertical => self.board[y + i][x] = ship.cell,
            }
        }

        Ok(())
    }

    pub fn all_ships_sunk(&mut self) -> bool {
        self.ships.iter().all(|ship| ship.sunk())
    }
}
