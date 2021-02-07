use nannou::Draw;

pub trait Nannou {
    fn draw(&self, draw: &Draw);
}
