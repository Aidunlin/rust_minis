mod dir;
mod pos;
mod snake;

use console::{Key, Term};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub use dir::Dir;
pub use pos::Pos;
pub use snake::{MoveResult, Snake};

fn clear_console() {
    Term::stdout().clear_screen().unwrap();
}

fn game_over() {
    clear_console();
    println!("GAME OVER");
    std::process::exit(0);
}

fn print_game(print_vec: Vec<Vec<char>>) {
    Term::stdout().move_cursor_to(0, 0).unwrap();

    for _ in 0..print_vec.len() * 2 + 3 {
        print!("-");
    }
    println!();

    for row in &print_vec {
        print!("|");
        for c in row {
            print!(" {}", c);
        }
        println!(" |");
    }

    for _ in 0..print_vec.len() * 2 + 3 {
        print!("-");
    }
    println!();

    io::stdout().flush().unwrap();
}

fn main() {
    let (width, height) = (10, 10);

    clear_console();

    let (tx, rx) = mpsc::channel();

    let mut snake = Snake::new(Dir::Right);

    let mut apple = snake.body[0];
    while snake.body.contains(&apple) {
        apple = Pos::random(width, height);
    }

    thread::spawn(move || {
        let mut last_key = Key::Unknown;
        loop {
            let read_key = Term::stdout().read_key().unwrap();
            if last_key != read_key {
                match read_key {
                    Key::ArrowUp => tx.send(Dir::Up).unwrap(),
                    Key::ArrowDown => tx.send(Dir::Down).unwrap(),
                    Key::ArrowLeft => tx.send(Dir::Left).unwrap(),
                    Key::ArrowRight => tx.send(Dir::Right).unwrap(),
                    _ => {}
                }
                last_key = read_key;
            }
        }
    });

    loop {
        if let Ok(direction) = rx.try_recv() {
            if direction != snake.dir.opposite() {
                snake.dir = direction;
            }
        }

        let mut last_pos = Pos::new(0, 0);
        match snake.move_step() {
            MoveResult::Moved(pos) => last_pos = pos,
            MoveResult::Collided => game_over(),
        }

        let mut print_vec = vec![vec![' '; width]; height];

        let mut ate_apple = false;

        for part in &snake.body {
            if part.x >= width || part.y >= height {
                game_over();
            }

            print_vec[part.y][part.x] = 'O';

            if *part == apple && !ate_apple {
                ate_apple = true;
            }
        }

        if ate_apple {
            while snake.body.contains(&apple) {
                apple = Pos::random(width, height);
            }
            snake.body.push(last_pos);
        }

        print_vec[apple.y][apple.x] = 'A';

        print_game(print_vec);

        thread::sleep(Duration::from_millis(150));
    }
}
