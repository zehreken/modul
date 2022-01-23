use crate::modul;

pub struct WindowMetronome {
    is_running: bool,
}

impl Default for WindowMetronome {
    fn default() -> Self {
        Self { is_running: false }
    }
}

impl WindowMetronome {
    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        let Self { is_running } = self;
        egui::Window::new("metronome").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!("time {}", 1234567890));
            let r = ui.checkbox(is_running, "run");
            if r.changed() {
                modul.switch_metronome(*is_running);
            }

            if modul.show_beat() {
                ui.label("dup");
            }
        });
    }
}
