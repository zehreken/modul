use super::modul;
use eframe::{
    egui::{self, color, Key},
    epi,
};
use egui::Ui;

struct EguiView {
    modul: modul::Modul,
    selected_tape: usize,
    name: String,
    age: u32,
}

impl Default for EguiView {
    fn default() -> Self {
        Self {
            modul: modul::Modul::new(),
            selected_tape: 0,
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl epi::App for EguiView {
    fn name(&self) -> &str {
        "modul"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            modul,
            selected_tape,
            name,
            age,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(name);
            });
            ui.add(egui::Slider::u32(age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                *age += 1;
            }
            ui.label(format!("Hello '{}', age {}", name, age));
            ui.horizontal(|ui| {
                if ui.selectable_label(*selected_tape == 0, "1").clicked() {
                    *selected_tape = 0;
                    modul.set_selected_tape(0);
                }
                if ui.selectable_label(*selected_tape == 1, "2").clicked() {
                    *selected_tape = 1;
                    modul.set_selected_tape(1);
                }
                if ui.selectable_label(*selected_tape == 2, "3").clicked() {
                    *selected_tape = 2;
                    modul.set_selected_tape(2);
                }
                if ui.selectable_label(*selected_tape == 3, "4").clicked() {
                    *selected_tape = 3;
                    modul.set_selected_tape(3);
                }
            });

            // Keyboard input
            for e in ui.input().events.iter() {
                match e {
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                    } => {
                        if !pressed {
                            match key {
                                Key::Num1 => {
                                    modul.set_selected_tape(0);
                                }
                                Key::Num2 => {
                                    modul.set_selected_tape(1);
                                }
                                Key::Num3 => {
                                    modul.set_selected_tape(2);
                                }
                                Key::Num4 => {
                                    modul.set_selected_tape(3);
                                }
                                Key::R => {
                                    modul.record();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

pub fn start() {
    eframe::run_native(Box::new(EguiView::default()));
}
