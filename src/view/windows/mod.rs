pub mod window_controls;
pub mod window_log;
pub mod window_metronome;
pub mod window_stats;
pub mod window_tapes;

use crate::modul::Modul;
use egui::{Key, Modifiers};
// use std::collections::HashMap;

trait Drawable {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut Modul);
}

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
    show_log: bool,
    window_log: window_log::WindowLog,
    // inventory: HashMap<bool, dyn Drawable>,
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
            show_log: true,
            window_log: window_log::WindowLog::default(),
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
                ui.checkbox(&mut self.show_log, "log");
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

            self.check_input(ui, modul);
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
            self.window_controls.draw(ctx, modul);
        }
        if self.show_log {
            self.window_log.draw(ctx, modul);
        }
    }

    fn check_input(&mut self, ui: &mut egui::Ui, modul: &mut Modul) {
        for e in ui.input().events.iter() {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
            } = e
            {
                if *pressed {
                    match key {
                        Key::Space => {
                            modul.record();
                        }
                        Key::C => {
                            if *modifiers == Modifiers::SHIFT {
                                modul.clear_all();
                            } else {
                                modul.clear();
                            }
                        }
                        Key::T => {
                            modul.record_playback();
                        }
                        Key::Y => {
                            modul.play_through();
                        }
                        Key::W => {
                            modul.write();
                        }
                        Key::Escape => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
