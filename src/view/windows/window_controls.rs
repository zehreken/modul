use crate::modul;

pub struct WindowControls {}

impl Default for WindowControls {
    fn default() -> Self {
        Self {}
    }
}

impl WindowControls {
    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut modul::Modul) {
        egui::Window::new("controls").show(ctx, |ui| {});
    }
}
