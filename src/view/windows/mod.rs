pub mod window_controls;
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
    is_play_through: bool,
    window_stats: window_stats::WindowStats,
    show_controls: bool,
    window_controls: window_controls::WindowControls,
}

impl Windows {
    pub fn new(egui_ctx: &egui::Context) -> Self {
        Self {
            show_tapes: true,
            window_tapes: window_tapes::WindowTapes::default(),
            show_metronome: false,
            window_metronome: window_metronome::WindowMetronome::new(egui_ctx),
            show_stats: false,
            is_play_through: false,
            window_stats: window_stats::WindowStats::default(),
            show_controls: false,
            window_controls: window_controls::WindowControls::default(),
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut Modul) {
        egui::TopBottomPanel::top("").show(ctx, |ui| {
            // egui::trace!(ui); // What does this do https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs
            ui.horizontal(|ui| {
                ui.label("modul ❤ ");
                ui.separator();
                ui.checkbox(&mut self.show_tapes, "tapes");
                if ui.checkbox(&mut self.show_metronome, "metronome").changed() {
                    modul.switch_metronome(self.show_metronome);
                };
                ui.checkbox(&mut self.show_stats, "stats");
                ui.checkbox(&mut self.show_controls, "controls");
                if ui
                    .checkbox(&mut self.is_play_through, "play through")
                    .changed()
                {
                    modul.play_through();
                }
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
        if self.show_controls {
            self.window_controls.draw(ctx);
        }
    }

    pub fn _check_input() {
        todo!("Move input stuff from window_tapes.rs")
    }
}
