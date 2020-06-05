use nannou::draw::Draw;

pub trait Nannou {
    fn draw(&self, draw: &Draw);
    fn update(&mut self);
}
