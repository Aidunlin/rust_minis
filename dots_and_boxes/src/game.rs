use crate::{cursor, Grid, GridExt, Player, Pos, Side};
use console::{Key, Term};

fn clear_console() {
    Term::stdout()
        .clear_screen()
        .expect("Failed to clear screen");
}

pub struct Game {
    pub grid: Grid,
    pub player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::new_grid(5),
            player: Player::A,
        }
    }

    pub fn play(&mut self) {
        clear_console();
        let mut pos = Pos::new(0, 0);
        let mut side = Side::Top;
        loop {
            cursor::move_to(Pos::new(0, 0));
            let player_style = self.player.get_style().bold();
            println!("{}", self.grid.to_string(pos));
            println!(
                "{}'s turn",
                player_style.apply_to(self.player.to_char(false))
            );
            cursor::get_pos(pos, side);
            match Term::stdout().read_key().expect("Failed to read key") {
                Key::ArrowUp => {
                    match side {
                        Side::Top => pos.y = self.checked_step(pos.y, false),
                        _ => side = Side::Top,
                    };
                }
                Key::ArrowDown => {
                    match side {
                        Side::Bottom => pos.y = self.checked_step(pos.y, true),
                        _ => side = Side::Bottom,
                    };
                }
                Key::ArrowLeft => {
                    match side {
                        Side::Left => pos.x = self.checked_step(pos.x, false),
                        _ => side = Side::Left,
                    };
                }
                Key::ArrowRight => {
                    match side {
                        Side::Right => pos.x = self.checked_step(pos.x, true),
                        _ => side = Side::Right,
                    };
                }
                Key::Enter => {
                    if self.grid.at(pos).side(side).is_empty() {
                        self.grid.fill_side(pos, side, self.player);
                        if self.grid.is_filled() {
                            clear_console();
                            println!("{} won!", player_style.apply_to(self.player.to_char(false)));
                            return;
                        }
                        self.player.next();
                    }
                }
                Key::Escape => break,
                _ => {}
            }
        }
        clear_console();
    }

    pub fn checked_step(&self, x: usize, pos_step: bool) -> usize {
        let limit = self.grid.len() - 1;
        let check = if pos_step {
            Some(if x >= limit { limit } else { x + 1 })
        } else {
            x.checked_sub(1)
        };
        match check {
            None => x,
            Some(x) => x,
        }
    }
}
