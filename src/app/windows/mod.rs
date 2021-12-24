pub mod window_metronome;
pub mod window_stats;
pub mod window_tapes;
use crate::modul::Modul;

pub struct Windows {
    show_tapes: bool,
    window_tapes: window_tapes::WindowTapes,
    show_metronome: bool,
    window_metronome: window_metronome::WindowMetronome,
    show_stats: bool,
    window_stats: window_stats::WindowStats,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            show_tapes: true,
            window_tapes: window_tapes::WindowTapes::default(),
            show_metronome: false,
            window_metronome: window_metronome::WindowMetronome::default(),
            show_stats: false,
            window_stats: window_stats::WindowStats::default(),
        }
    }

    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut Modul) {
        egui::TopBottomPanel::top("").show(ctx, |ui| {
            // egui::trace!(ui); // What does this do https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs
            ui.horizontal(|ui| {
                ui.label("modul ❤ ");
                ui.separator();
                ui.checkbox(&mut self.show_tapes, "tapes");
                ui.checkbox(&mut self.show_metronome, "metronome");
                ui.checkbox(&mut self.show_stats, "stats");
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                }
            });
        });

        if self.show_tapes {
            self.window_tapes.draw(ctx, modul);
        }
        if self.show_metronome {
            self.window_metronome.draw(ctx, modul);
        }
        if self.show_stats {
            self.window_stats.draw(ctx, modul);
        }
    }

    pub fn _check_input() {
        todo!("Move input stuff from window_tapes.rs")
    }
}
