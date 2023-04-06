use nannou::prelude::*;

pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub is_alive: bool,
}

pub trait Drawable {
    fn draw(&self, draw: &Draw);
}

//impl Drawable for Cell {
//    fn draw(&self, draw: &Draw) {
//        let cell_color = if self.is_alive == true { BLACK } else { WHITE };
//
//       draw.rect().x_y(self.x, self.y);
//    }
//}
