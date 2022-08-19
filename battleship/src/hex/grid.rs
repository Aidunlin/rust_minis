use super::*;

#[derive(Clone)]
pub enum Cell {
    Empty,
    Hit,
    Miss,
    Ship(ShipType),
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => '-',
            Cell::Hit => '*',
            Cell::Miss => '/',
            Cell::Ship(ship_type) => match ship_type {
                ShipType::AircraftCarrier => 'C',
                ShipType::Battleship => 'B',
                ShipType::WeaponsPlatform => 'W',
                ShipType::Submarine => 'S',
                ShipType::PatrolBoat => 'P',
            },
        }
    }
}

pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let mut cells = Vec::new();

        for y in 0..size {
            cells.push(vec![Cell::Empty; y + 1]);
        }

        for y in 0..(size * 2) - 3 {
            if y % 2 == 0 {
                cells.push(vec![Cell::Empty; size - 1]);
            } else {
                cells.push(vec![Cell::Empty; size]);
            }
        }

        for y in 0..size - 1 {
            cells.push(vec![Cell::Empty; size - y]);
        }

        Self { size, cells }
    }

    pub fn display(&self) {
        println!();

        let mut iterator = self.cells.iter();

        for y in 0..self.size {
            let row = iterator.next().unwrap();
            print!("{}", "   ".repeat(self.size - 1 - y));
            for cell in row {
                print!("{}     ", cell.to_char());
            }
            println!();
        }

        for y in 0..(self.size * 2) - 3 {
            let row = iterator.next().unwrap();
            if y % 2 == 0 {
                print!("   ");
            }
            for cell in row {
                print!("{}     ", cell.to_char());
            }
            println!();
        }

        for y in 0..self.size - 1 {
            let row = iterator.next().unwrap();
            print!("{}", "   ".repeat(y));
            for cell in row {
                print!("{}     ", cell.to_char());
            }
            println!();
        }
    }
}
