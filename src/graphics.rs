use nannou::prelude::*;
use nannou::Draw;
pub struct Tape {
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Tape {
    pub fn draw(&self, draw: &Draw) {
        draw.rect()
            .w_h(100.0, 100.0)
            .x_y(self.pos_x, self.pos_y)
            .color(DARKCYAN);
    }
}
