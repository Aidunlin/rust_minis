mod game;
mod player;
mod grid;
mod rot;
mod ship;

use game::Game;
pub use player::Player;
pub use grid::{Cell, Grid};
pub use rot::{Direction, Rotation};
pub use ship::{Ship, ShipType};

pub fn play() {
    Game::new(8).play();
}
