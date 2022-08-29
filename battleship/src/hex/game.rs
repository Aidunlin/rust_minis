use rand::Rng;
use super::*;

pub struct Game {
    pub grid: Grid,
    pub player_1: Player,
    pub player_2: Player,
}

impl Game {
    pub fn new(size: usize) -> Game {
        Game {
            grid: Grid::new(size),
            player_1: Player::new(),
            player_2: Player::new(),
        }
    }

    pub fn play(&self) {
        self.grid.display();
    }

    pub fn place_ship(&mut self, ship: Ship, direction: Direction, x: usize, y: usize) {
        let cells = self.gen_cells(ship, direction, x, y);
    }

    pub fn place_ships_randomly(&mut self, player: Player, upper_half: bool) {
        let mut rng = rand::thread_rng();
        for ship in player.ships.iter() {
            let direction: Direction = rand::random();
            let x = rng.gen_range(0..self.grid.cells.len());
            let y = rng.gen_range(0..self.grid.cells[x].len());
        }
    }

    pub fn gen_cells(&self, ship: Ship, direction: Direction, x: usize, y: usize) -> Vec<(usize, usize)> {
        match ship.ship_type {
            ShipType::AircraftCarrier => match direction {
                Direction::North => vec![(), (), (), ()],
                Direction::NorthEast => vec![(), (), (), ()],
                Direction::SouthEast => vec![(), (), (), ()],
                Direction::South => vec![(), (), (), ()],
                Direction::SouthWest => vec![(), (), (), ()],
                Direction::NorthWest => vec![(), (), (), ()],
            },
            ShipType::Battleship => match direction {
                Direction::North => vec![(), (), ()],
                Direction::NorthEast => vec![(), (), ()],
                Direction::SouthEast => vec![(), (), ()],
                Direction::South => vec![(), (), ()],
                Direction::SouthWest => vec![(), (), ()],
                Direction::NorthWest => vec![(), (), ()],
            },
            ShipType::WeaponsPlatform => match direction {
                Direction::North => vec![(), ()],
                Direction::NorthEast => vec![(), ()],
                Direction::SouthEast => vec![(), ()],
                Direction::South => vec![(), ()],
                Direction::SouthWest => vec![(), ()],
                Direction::NorthWest => vec![(), ()],
            },
            ShipType::Submarine => match direction {
                Direction::North | Direction::South => vec![(x, y - 1), (x, y + 1)],
                Direction::NorthEast | Direction::SouthWest => vec![(), ()],
                Direction::SouthEast | Direction::NorthWest => vec![(), ()],
            },
            ShipType::PatrolBoat => match direction {
                Direction::North => vec![(x, y - 1)],
                Direction::NorthEast => vec![()],
                Direction::SouthEast => vec![()],
                Direction::South => vec![(x, y + 1)],
                Direction::SouthWest => vec![()],
                Direction::NorthWest => vec![()],
            },
        }
    }
}
