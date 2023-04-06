use nannou::{prelude::*, rand::seq::index};

use crate::cell::*;
use crate::configs::*;

pub struct World {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    cell_size_px: u32,
}

impl World {
    pub fn new(width: u32, height: u32, cell_size_px: u32) -> World {
        let mut cells_new: Vec<Cell> = vec![];

        for x in 0..CELL_COLUMNS {
            for y in 0..CELL_ROWS {
                if x == y {
                    cells_new.push(Cell {
                        x: x as f32,
                        y: y as f32,
                        is_alive: true,
                    });
                } else {
                    cells_new.push(Cell {
                        x: x as f32,
                        y: y as f32,
                        is_alive: false,
                    });
                }
            }
        }

        World {
            cells: cells_new,
            width: width,
            height: height,
            cell_size_px: cell_size_px,
        }
    }

    pub fn update(&self) {}
}

impl Drawable for World {
    fn draw(&self, draw: &Draw) {
        for cell in self.cells.iter() {
            let cell_color = if cell.is_alive == true { BLACK } else { WHITE };
            let mut x_pos = cell.x * (CELL_PX_SIZE as f32);
            x_pos -= (SCREEN_WIDTH / 2) as f32;
            let mut y_pos = cell.y * (CELL_PX_SIZE as f32);
            y_pos -= (SCREEN_HEIGHT / 2) as f32;

            draw.rect()
                .x_y(x_pos, y_pos)
                .color(cell_color)
                .width(CELL_PX_SIZE as f32 - 0.5)
                .height(CELL_PX_SIZE as f32 - 0.5)
                .stroke_color(BLACK)
                .stroke_weight(0.5);
        }
    }
}
