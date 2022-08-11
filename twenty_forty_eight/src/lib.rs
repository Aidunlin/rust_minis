use rand::Rng;

pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vector(direction: Direction) -> Vector<isize> {
        match direction {
            Direction::Up => Vector { x: 0, y: -1 },
            Direction::Down => Vector { x: 0, y: 1 },
            Direction::Left => Vector { x: -1, y: 0 },
            Direction::Right => Vector { x: 1, y: 0 },
        }
    }
}

pub struct TwentyFortyEight {
    pub grid: Vec<Vec<u16>>,
    pub size: usize,
}

impl TwentyFortyEight {
    pub fn new(size: usize) -> Self {
        Self {
            grid: vec![vec![0; size]; size],
            size,
        }
    }

    pub fn available(&self) -> Vec<Vector<usize>> {
        let mut cells = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    cells.push(Vector { x, y });
                }
            }
        }
        cells
    }

    pub fn is_empty(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|cell| *cell == 0))
    }

    pub fn place_random_cell(&mut self) {
        let cells = self.available();
        let choice = &cells[rand::thread_rng().gen_range(0..cells.len())];
        let value_is_four = rand::thread_rng().gen_ratio(1, 10);
        self.grid[choice.y][choice.x] = if value_is_four { 4 } else { 2 };
    }

    pub fn initialize(&mut self, starting_tiles: usize) {
        for _ in 0..starting_tiles {
            self.place_random_cell();
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 1..self.size {
                        let mut final_y = 0;
                        while final_y < self.size && self.grid[final_y][x] != 0 {
                            final_y += 1;
                        }
                        if final_y < self.size - 1 {
                            self.grid[final_y][x] = self.grid[y][x];
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in (0..self.size - 1).rev() {
                        let mut final_y = self.size - 1;
                        while final_y > 0 && self.grid[final_y][x] != 0 {
                            final_y -= 1;
                        }
                        if final_y != 0 {
                            self.grid[final_y][x] = self.grid[y][x];
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..self.size {
                    for x in 1..self.size {
                        let mut final_x = 0;
                        while final_x < self.size && self.grid[y][final_x] != 0 {
                            final_x += 1;
                        }
                        if final_x < self.size - 1 {
                            self.grid[y][final_x] = self.grid[y][x];
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..self.size {
                    for x in (0..self.size - 1).rev() {
                        let mut final_x = self.size - 1;
                        while final_x > 0 && self.grid[y][final_x] != 0 {
                            final_x -= 1;
                        }
                        if final_x != 0 {
                            self.grid[y][final_x] = self.grid[y][x];
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
        }
    }

    pub fn merge(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                for x in 0..self.size {
                    for y in 0..self.size - 1 {
                        if self.grid[y][x] != 0 && self.grid[y][x] == self.grid[y + 1][x] {
                            self.grid[y + 1][x] = self.grid[y][x] * 2;
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Down => {
                for x in 0..self.size {
                    for y in 0..self.size - 1 {
                        if self.grid[y][x] != 0 && self.grid[y][x] == self.grid[y + 1][x] {
                            self.grid[y + 1][x] = self.grid[y][x] * 2;
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Left => {
                for y in 0..self.size {
                    for x in 0..self.size - 1 {
                        if self.grid[y][x] != 0 && self.grid[y][x] == self.grid[y][x + 1] {
                            self.grid[y][x + 1] = self.grid[y][x] * 2;
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
            Direction::Right => {
                for y in 0..self.size {
                    for x in 0..self.size - 1 {
                        if self.grid[y][x] != 0 && self.grid[y][x] == self.grid[y][x + 1] {
                            self.grid[y][x + 1] = self.grid[y][x] * 2;
                            self.grid[y][x] = 0;
                        }
                    }
                }
            }
        }
    }

    pub fn do_move(&mut self, direction: Direction) {
        let before = self.grid.clone();
        self.shift(direction);
        self.merge(direction);
        self.shift(direction);
        let matching = before
            .iter()
            .flatten()
            .zip(self.grid.clone().iter().flatten())
            .filter(|&(before, after)| before == after)
            .count();
        if matching < self.size * self.size {
            self.place_random_cell();
        }
    }

    pub fn display(&self) {
        print!("+");
        for _ in 0..self.size {
            print!("{:->6}", "+");
        }
        println!();

        self.grid.iter().for_each(|row| {
            print!("|");
            row.iter().for_each(|cell| {
                if *cell == 0 {
                    print!("{:^5}|", "");
                } else {
                    print!("{:^5}|", cell);
                }
            });
            println!();

            print!("+");
            for _ in 0..self.size {
                print!("{:->6}", "+");
            }
            println!();
        });
    }
}
