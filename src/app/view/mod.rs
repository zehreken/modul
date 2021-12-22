pub mod window_metronome;
pub mod window_stats;
pub mod window_tapes;
use crate::modul::Modul;

pub struct Windows {
    window_tapes: window_tapes::WindowTapes,
    window_metronome: window_metronome::WindowMetronome,
    window_stats: window_stats::WindowStats,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            window_tapes: window_tapes::WindowTapes::default(),
            window_metronome: window_metronome::WindowMetronome::default(),
            window_stats: window_stats::WindowStats::default(),
        }
    }

    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut Modul) {
        self.window_tapes.draw(ctx, modul);
        self.window_metronome.draw(ctx, modul);
        self.window_stats.draw(ctx, modul);
    }
}
