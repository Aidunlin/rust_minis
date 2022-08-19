mod game;
mod player;
mod ship;

pub use console::{Key, Term};
pub use game::Game;
pub use player::{Axis, Player, PlayerType};
pub use ship::{Cell, Ship};

pub fn play() {
    Game::new(10).play();
}
