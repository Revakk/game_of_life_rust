use nannou::prelude::*;

#[derive(Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub is_alive: bool,
}

pub trait Drawable {
    fn draw(&self, draw: &Draw);
}

pub enum CellState {
    ALIVE,
    DEAD,
}

pub fn cell_state_from_neighbour_count(cell: &Cell, n_count: u32) -> CellState {
    if cell.is_alive {
        match n_count {
            2..=3 => CellState::ALIVE,
            _ => CellState::DEAD,
        }
    } else {
        match n_count {
            3 => CellState::ALIVE,
            _ => CellState::DEAD,
        }
    }
}

//impl Drawable for Cell {
//    fn draw(&self, draw: &Draw) {
//        let cell_color = if self.is_alive == true { BLACK } else { WHITE };
//
//       draw.rect().x_y(self.x, self.y);
//    }
//}
