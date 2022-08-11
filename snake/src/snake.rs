use crate::{Dir, Pos};

pub struct Snake {
    pub dir: Dir,
    pub body: Vec<Pos>,
}

pub enum MoveResult {
    Moved(Pos),
    Collided,
}

impl Snake {
    pub fn new(dir: Dir) -> Self {
        Self {
            dir,
            body: vec![Pos { x: 0, y: 0 }],
        }
    }

    pub fn move_step(&mut self) -> MoveResult {
        let mut last_pos = self.body[0];

        match self.dir {
            Dir::Up => match self.body[0].y.checked_sub(1) {
                None => return MoveResult::Collided,
                Some(y) => self.body[0].y = y,
            },
            Dir::Down => self.body[0].y += 1,
            Dir::Left => match self.body[0].x.checked_sub(1) {
                None => return MoveResult::Collided,
                Some(x) => self.body[0].x = x,
            },
            Dir::Right => self.body[0].x += 1,
        }

        let mut collided = false;
        let mut new_pos = self.body[0];
        for i in 1..self.body.len() {
            new_pos = self.body[i];
            if new_pos == self.body[0] {
                collided = true;
            }
            self.body[i] = last_pos;
            last_pos = new_pos;
        }

        if collided {
            MoveResult::Collided
        } else {
            MoveResult::Moved(new_pos)
        }
    }
}
