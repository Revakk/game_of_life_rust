use nannou::Draw;

use crate::{
    cell::Cell, cell::CellState, cell::Drawable, world::World, CELL_COLUMNS, CELL_PX_SIZE,
    CELL_ROWS,
};
use std::{thread, time};

pub struct GOL {
    world: World,
    update_time: u64,
    editor_mode: bool,
}

impl GOL {
    pub fn new() -> GOL {
        GOL {
            world: World::new(CELL_COLUMNS, CELL_ROWS, CELL_PX_SIZE),
            update_time: 1,
            editor_mode: true,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.world.draw(&draw);
    }

    fn update_world(&mut self) {
        let time_to_sleep = std::time::Duration::from_secs(self.update_time);
        std::thread::sleep(time_to_sleep);
        self.world.update();
    }

    fn edit_world(&self) {}

    pub fn update(&mut self) {
        match self.editor_mode {
            false => self.update_world(),
            true => self.edit_world(),
        }
    }
}
