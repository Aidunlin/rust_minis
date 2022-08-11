use console::{Key, Term};
use twenty_forty_eight::{Direction, TwentyFortyEight};

fn main() {
    Term::stdout().clear_screen().unwrap();
    let mut game = TwentyFortyEight::new(4);
    game.initialize(2);

    loop {
        Term::stdout().move_cursor_to(0, 0).unwrap();
        game.display();
        
        match Term::stdout().read_key().unwrap() {
            Key::ArrowLeft => game.do_move(Direction::Left),
            Key::ArrowRight => game.do_move(Direction::Right),
            Key::ArrowUp => game.do_move(Direction::Up),
            Key::ArrowDown => game.do_move(Direction::Down),
            Key::Escape => break,
            _ => (),
        }
    }

    Term::stdout().clear_screen().unwrap();
}
