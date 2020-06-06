use super::beat_controller::*;
use nannou::prelude::*;
use nannou::Draw;
pub struct Tape {
    pub pos_x: f32,
    pub pos_y: f32,
}

impl Tape {
    pub fn draw(&self, draw: &Draw, beat_controller: &BeatController) {
        let radian = -2.0 * std::f32::consts::PI
            + (beat_controller.bar_timer as f32
                / (beat_controller.beat_period as f32 * beat_controller.beat_count as f32))
                * std::f32::consts::PI
                * 2.0;
        draw.ellipse()
            .w_h(128.0, 128.0)
            .x_y(self.pos_x, self.pos_y)
            .color(DARKCYAN);
        draw.ellipse()
            .w_h(32.0, 32.0)
            .x_y(
                self.pos_x + radian.cos() * 48.0,
                self.pos_y + radian.sin() * 48.0,
            )
            .color(BLACK);
    }
}
