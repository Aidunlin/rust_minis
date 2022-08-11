use console::Term;
use rand::Rng;
use std::{fmt::Debug, io, str::FromStr};

/// "Clears" the terminal.
pub fn clear_console() {
    Term::stdout().clear_screen().unwrap();
}

/// Pauses the terminal.
pub fn pause_console() {
    println!("Press any key to continue");
    Term::stdout().read_key().unwrap();
}

/// Reads and returns a number of type `T` from `stdin`.
pub fn input_number<T>() -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    buffer.trim().parse::<T>().unwrap()
}

/// Generates a random `usize` between `start` and `end`.
pub fn rand_range(start: usize, end: usize) -> usize {
    rand::thread_rng().gen_range(start..end)
}
