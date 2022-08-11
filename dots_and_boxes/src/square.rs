use crate::{Player, Pos};
use console::Style;

#[derive(Clone, Copy, PartialEq)]
pub enum FillState {
    Empty,
    Filled(Player),
}

impl FillState {
    pub fn to_char(self, is_lower: bool) -> char {
        match self {
            FillState::Empty => ' ',
            FillState::Filled(player) => player.to_char(is_lower),
        }
    }

    pub fn get_style(self, bg_colored: bool) -> Style {
        let mut style = Style::default();
        if let FillState::Filled(player) = self {
            style = player.get_style();
            if bg_colored {
                style = style.reverse().on_black();
            }
        }
        style
    }

    pub fn is_empty(self) -> bool {
        self == FillState::Empty
    }

    pub fn is_filled(self) -> bool {
        !self.is_empty()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    pub fn get_opposite(self) -> Side {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    pub fn switch(&mut self, side: Side) {
        *self = if *self == side {
            self.get_opposite()
        } else {
            side
        };
    }
}

pub struct Square {
    pub pos: Pos,
    pub state: FillState,
    pub top: FillState,
    pub bottom: FillState,
    pub left: FillState,
    pub right: FillState,
}

impl Square {
    pub fn new(pos: Pos) -> Self {
        Self {
            pos,
            state: FillState::Empty,
            top: FillState::Empty,
            bottom: FillState::Empty,
            left: FillState::Empty,
            right: FillState::Empty,
        }
    }

    pub fn side(&mut self, side: Side) -> &mut FillState {
        match side {
            Side::Top => &mut self.top,
            Side::Bottom => &mut self.bottom,
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    pub fn fill_side(&mut self, side: Side, player: Player) {
        let player = FillState::Filled(player);
        match side {
            Side::Top => self.top = player,
            Side::Bottom => self.bottom = player,
            Side::Left => self.left = player,
            Side::Right => self.right = player,
        }
        if self.top.is_filled()
            && self.left.is_filled()
            && self.bottom.is_filled()
            && self.right.is_filled()
        {
            self.state = player;
        }
    }

    pub fn is_filled(&self) -> bool {
        self.state.is_filled()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn side_row_to_string(&self, col: usize, side: FillState) -> String {
        let mut output = String::new();
        if col == 0 {
            output += "+";
        }
        if side.is_filled() {
            let side_style = side.get_style(false);
            let side_str = format!("-{}-", side.to_char(true));
            output += format!("{}+", side_style.apply_to(side_str)).as_str();
        } else {
            output += "   +";
        }
        output
    }

    pub fn middle_to_string(&self, col: usize, selected_pos: bool) -> String {
        let left_style = self.left.get_style(false);
        let mut state_style = self.state.get_style(true).bold();
        if selected_pos {
            state_style = state_style.on_color256(8);
        }
        let right_style = self.right.get_style(false);
        let left_char = self.left.to_char(true);
        let state_str = format!(" {} ", self.state.to_char(false));
        let right_char = self.right.to_char(true);
        if col == 0 {
            format!(
                "{}{}{}",
                left_style.apply_to(left_char),
                state_style.apply_to(state_str),
                right_style.apply_to(right_char),
            )
        } else {
            format!(
                "{}{}",
                state_style.apply_to(state_str),
                right_style.apply_to(right_char)
            )
        }
    }
}
