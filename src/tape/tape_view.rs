use super::beat_controller::*;
use nannou::prelude::*;
use nannou::Draw;
pub struct TapeView {
    pub pos_x: f32,
    pub pos_y: f32,
    pub is_selected: bool,
}

impl TapeView {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            pos_x,
            pos_y,
            is_selected: false,
        }
    }
    pub fn draw(&self, draw: &Draw, position: f32) {
        let radian = -2.0 * std::f32::consts::PI + position * std::f32::consts::PI * 2.0;
        draw.ellipse()
            .w_h(128.0, 128.0)
            .x_y(self.pos_x, self.pos_y)
            .color(if self.is_selected { RED } else { DARKCYAN });
        draw.ellipse()
            .w_h(32.0, 32.0)
            .x_y(
                self.pos_x + radian.cos() * 44.0,
                self.pos_y + radian.sin() * 44.0,
            )
            .color(BLACK);
    }
}
