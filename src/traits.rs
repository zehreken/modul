use nannou::Draw;

pub trait Nannou {
    fn draw(&self, draw: &Draw);
    fn update(&mut self, delta_time: i32);
}
