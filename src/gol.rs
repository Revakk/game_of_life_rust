use nannou::Draw;

use crate::{cell::Drawable, world::World, CELL_COLUMNS, CELL_PX_SIZE, CELL_ROWS};

pub struct GOL {
    world: World,
}

impl GOL {
    pub fn new() -> GOL {
        GOL {
            world: World::new(CELL_COLUMNS, CELL_ROWS, CELL_PX_SIZE),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.world.draw(&draw);
    }

    pub fn update(&mut self) {
        self.world.update();
    }
}
