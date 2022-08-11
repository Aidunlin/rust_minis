mod game;
mod player;
mod ship;
mod util;

pub use game::Game;
pub use player::{Axis, Player, PlayerType};
pub use ship::{Cell, Ship};

pub fn play() {
    Game::new(10).play();
}
