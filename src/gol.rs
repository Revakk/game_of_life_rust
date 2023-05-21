use crate::configs::*;
use nannou::Draw;
use nannou::{glam::Vec2, prelude::MouseButton};

use crate::{
    cell::Cell, cell::CellState, cell::Drawable, world::World, CELL_COLUMNS, CELL_PX_SIZE,
    CELL_ROWS,
};

pub struct GOL {
    world: World,
    update_time: u64,
    editor_mode: bool,
    mouse_position: Vec2,
    pub active_mouse_button: MouseButton,
    pub is_mb_active: bool,
}

impl GOL {
    pub fn new() -> GOL {
        GOL {
            world: World::new(CELL_COLUMNS, CELL_ROWS, CELL_PX_SIZE),
            update_time: 1,
            editor_mode: true,
            mouse_position: Vec2::new(0.0, 0.0),
            active_mouse_button: MouseButton::Left,
            is_mb_active: false,
        }
    }

    pub fn set_mouse_pos(&mut self, new_mouse_pos: Vec2) {
        if self.editor_mode == true {
            self.mouse_position = new_mouse_pos;
            //println!("new mouse pos: {}", self.mouse_position);
        }
    }

    pub fn change_cell_state(&mut self) {
        if self.editor_mode == true && self.is_mb_active == true {
            let pos = self.get_cell_from_coords(self.mouse_position);
            if self.world.position_in_range(pos) {
                match self.active_mouse_button {
                    MouseButton::Left => {
                        self.world.change_position_cell_state(pos, CellState::ALIVE)
                    }
                    MouseButton::Right => {
                        self.world.change_position_cell_state(pos, CellState::DEAD)
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.world.draw(&draw);
    }

    fn get_cell_from_coords(&self, coords: Vec2) -> usize {
        //let x = (coords.x / CELL_PX_SIZE as f32) as i32 + (self.world.get_world_width() / 2);
        //let y = (coords.y / CELL_PX_SIZE as f32) as i32 + (self.world.get_world_height() / 2);

        let x =
            (coords.x / CELL_PX_SIZE as f32).round() as i32 + (self.world.get_world_width() / 2);
        let y =
            (coords.y / CELL_PX_SIZE as f32).round() as i32 + (self.world.get_world_height() / 2);

        //println!("mouse on x:{} , y: {}", x, y);

        ((self.world.get_world_width() * x) + y) as usize
    }

    fn update_world(&mut self) {
        let time_to_sleep = std::time::Duration::from_secs(self.update_time);
        std::thread::sleep(time_to_sleep);
        self.world.update();
    }

    pub fn update(&mut self) {
        match self.editor_mode {
            false => self.update_world(),
            true => (),
        }
    }
    pub fn switch_mode(&mut self) {
        self.editor_mode = !self.editor_mode;
    }
}
