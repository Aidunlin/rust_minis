use super::*;

pub struct Player {
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn new() -> Self {
        Self { ships: Ship::new_set() }
    }
}
