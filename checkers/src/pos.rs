pub enum Dir {
    NW,
    NE,
    SW,
    SE,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl Pos<usize> {
    pub fn move_by(&self, dir: &Dir) -> Pos<usize> {
        let pos = Pos {
            x: self.x as isize,
            y: self.y as isize,
        };
        let by = match dir {
            Dir::NW => Pos { x: -1, y: -1 },
            Dir::NE => Pos { x: 1, y: -1 },
            Dir::SW => Pos { x: -1, y: 1 },
            Dir::SE => Pos { x: 1, y: 1 },
        };
        Pos {
            x: (pos.x + by.x) as usize,
            y: (pos.y + by.y) as usize,
        }
    }

    pub fn is_valid(&self, board_size: usize) -> bool {
        self.x < board_size && self.y < board_size
    }
}
