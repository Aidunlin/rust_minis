#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Hit,
    Miss,
    Ship(char),
}

#[derive(Clone, Copy)]
pub struct Ship {
    pub cell: Cell,
    pub name: &'static str,
    pub size: usize,
    pub hits: usize,
}

impl Ship {
    pub fn new(cell: char, name: &'static str, size: usize) -> Self {
        Self {
            cell: Cell::Ship(cell),
            name,
            size,
            hits: 0,
        }
    }

    pub fn sunk(&self) -> bool {
        self.hits >= self.size
    }
}
