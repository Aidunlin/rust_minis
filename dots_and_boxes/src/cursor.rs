use crate::{Pos, Side};
use console::Term;

pub fn move_to(pos: Pos) -> Pos {
    Term::stdout()
        .move_cursor_to(pos.x, pos.y)
        .expect("Failed to move cursor");
    pos
}

pub fn get_pos(pos: Pos, side: Side) -> Pos {
    let pos = Pos::new(pos.x * 4 + 3, pos.y * 2 + 2);
    let output = match side {
        Side::Top => Pos::new(pos.x, pos.y - 1),
        Side::Bottom => Pos::new(pos.x, pos.y + 1),
        Side::Left => Pos::new(pos.x - 2, pos.y),
        Side::Right => Pos::new(pos.x + 2, pos.y),
    };
    move_to(output)
}
