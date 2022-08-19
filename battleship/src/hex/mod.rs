mod player;
mod grid;
mod rot;
mod ship;

pub use grid::{Cell, Grid};
pub use rot::{Direction, Rotation};
pub use ship::{Ship, ShipType};

pub fn play() {
    let grid = Grid::new(8);
    grid.display();
}
