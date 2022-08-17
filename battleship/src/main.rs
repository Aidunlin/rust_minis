mod game;
mod player;
mod ship;

pub use console::{Key, Term};
use game::Game;
pub use player::{Axis, Player, PlayerType};
pub use ship::{Cell, Ship};

fn main() {
    Game::new(10).play();
}

pub mod util {
    use console::{Key, Term};
    use rand::Rng;
    use std::{fmt, io, str};

    pub fn clear_console() {
        match Term::stdout().clear_screen() {
            Ok(_) => (),
            Err(_) => println!("{}", "\n".repeat(10)),
        }
    }

    pub fn pause_console() {
        println!("Press any key to continue");
        Term::stdout().read_key().expect("Failed to pause console");
    }

    pub fn input_number<T>() -> T
    where
        T: str::FromStr,
        T::Err: fmt::Debug,
    {
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        buffer.trim().parse::<T>().unwrap()
    }

    pub fn read_key() -> Key {
        Term::stdout().read_key().expect("Failed to read key")
    }

    pub fn rand_range(start: usize, end: usize) -> usize {
        rand::thread_rng().gen_range(start..end)
    }

    pub fn move_cursor(x: usize, y: usize) {
        Term::stdout()
            .move_cursor_to(x, y)
            .expect("Failed to move cursor");
    }
}
