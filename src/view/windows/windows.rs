use egui::{Key, Modifiers};
// use std::collections::HashMap;

pub trait Drawable {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut super::Modul);
}

pub struct Windows {
    show_tapes: bool,
    window_tapes: super::window_tapes::WindowTapes,
    show_metronome: bool,
    window_metronome: super::window_metronome::WindowMetronome,
    show_stats: bool,
    is_play_through: bool,
    window_stats: super::window_stats::WindowStats,
    show_controls: bool,
    window_controls: super::window_controls::WindowControls,
    show_log: bool,
    window_log: super::window_log::WindowLog,
    show_settings: bool,
    window_settings: super::window_settings::WindowSettings,
    // inventory: HashMap<bool, dyn Drawable>,
}

impl Windows {
    pub fn new(egui_ctx: &egui::Context) -> Self {
        Self {
            show_tapes: true,
            window_tapes: super::window_tapes::WindowTapes::default(),
            show_metronome: false,
            window_metronome: super::window_metronome::WindowMetronome::new(egui_ctx),
            show_stats: false,
            is_play_through: false,
            window_stats: super::window_stats::WindowStats::default(),
            show_controls: false,
            window_controls: super::window_controls::WindowControls::default(),
            show_log: false,
            window_log: super::window_log::WindowLog::default(),
            show_settings: false,
            window_settings: super::window_settings::WindowSettings::default(),
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut super::Modul) {
        egui::TopBottomPanel::top("").show(ctx, |ui| {
            // egui::trace!(ui); // What does this do https://github.com/emilk/egui/blob/master/egui_demo_lib/src/wrap_app.rs
            ui.horizontal(|ui| {
                ui.label("modul ❤ ");
                ui.separator();

                ui.menu_button("Window", |ui| {
                    ui.checkbox(&mut self.show_tapes, "Tapes");
                    if ui.checkbox(&mut self.show_metronome, "Metronome").changed() {
                        modul.switch_metronome(self.show_metronome);
                    };
                    ui.checkbox(&mut self.show_stats, "Stats");
                    ui.checkbox(&mut self.show_controls, "Controls");
                    ui.checkbox(&mut self.show_log, "Log");
                    ui.checkbox(&mut self.show_settings, "Settings");
                });
                if ui
                    .checkbox(&mut self.is_play_through, "play through")
                    .changed()
                {
                    modul.play_through();
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
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
        if self.show_settings {
            self.window_settings.draw(ctx, modul);
        }
    }

    fn check_input(&mut self, ui: &mut egui::Ui, modul: &mut super::Modul) {
        if ui.input(|i| i.key_pressed(Key::Space)) {
            modul.record();
        }
        if ui.input(|i| i.key_pressed(Key::C)) {
            modul.clear();
        }
        if ui.input(|i| i.key_pressed(Key::C) && i.modifiers == Modifiers::SHIFT) {
            modul.clear_all();
        }
        if ui.input(|i| i.key_pressed(Key::T)) {
            modul.record_playback();
        }
        if ui.input(|i| i.key_pressed(Key::Y)) {
            modul.play_through();
        }
        if ui.input(|i| i.key_pressed(Key::W)) {
            modul.write();
        }
        if ui.input(|i| i.key_pressed(Key::Escape)) {
            std::process::exit(0);
        }
    }
}
