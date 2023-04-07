use nannou::Draw;

use crate::{cell::Drawable, world::World, CELL_COLUMNS, CELL_PX_SIZE, CELL_ROWS};
use std::{thread, time};

pub struct GOL {
    world: World,
    update_time: u64,
}

impl GOL {
    pub fn new() -> GOL {
        GOL {
            world: World::new(CELL_COLUMNS, CELL_ROWS, CELL_PX_SIZE),
            update_time: 1,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.world.draw(&draw);
    }

    pub fn update(&mut self) {
        self.world.update();

        let time_to_sleep = std::time::Duration::from_secs(self.update_time);
        std::thread::sleep(time_to_sleep);
    }
}
