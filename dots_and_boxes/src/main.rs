pub mod cursor;
pub mod game;
pub mod grid;
pub mod player;
pub mod pos;
pub mod square;

pub use game::Game;
pub use grid::{Grid, GridExt};
pub use player::Player;
pub use pos::Pos;
pub use square::{Side, Square};

fn main() {
    let mut game = Game::new();
    game.play();
}
