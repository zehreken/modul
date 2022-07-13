use std::collections::VecDeque;

use crate::modul;

use super::Drawable;

#[derive(Default)]
pub struct WindowLog {
    message_history: VecDeque<String>,
}

impl Drawable for WindowLog {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut modul::Modul) {}
}
