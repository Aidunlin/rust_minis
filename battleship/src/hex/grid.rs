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
            Cell::Miss => '\'',
            Cell::Ship(ship_type) => ship_type.to_char(),
        }
    }
}

pub struct Grid {
    pub size: usize,
    /// Cells organized column-first (index x, then y)
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let mut cells = Vec::new();

        let max_size = 2 * (size - 1);

        for x in 0..2 * size - 1 {
            let mut column = Vec::new();

            let column_size = max_size + 1 - (x as isize - size as isize + 1).abs() as usize;

            for _ in 0..max_size.min(column_size) {
                column.push(Cell::Empty);
            }

            cells.push(column);
        }

        Self { size, cells }
    }

    pub fn display(&self) {
        let mut output = vec![vec![' '; 6 * self.size]; 4 * (self.size - 1)];

        for (x, column) in self.cells.iter().enumerate() {
            for (y, cell) in column.iter().enumerate() {
                let before_padding = if x <= self.size - 1 {
                    self.size - x - 1
                } else {
                    x - self.size + 1
                };

                output[y * 2 + before_padding][x * 3] = cell.to_char();
            }
        }

        for row in output.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        match self.cells.get(x) {
            Some(column) => column.get(y).is_some(),
            None => false,
        }
    }
}
