use super::modul;
use eframe::{
    egui::{self, Key, *},
    epi,
};
use std::time::Instant;

struct EguiView {
    instant: Instant,
    modul: modul::Modul,
    selected_tape: usize,
}

impl Default for EguiView {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
            modul: modul::Modul::new(),
            selected_tape: 0,
        }
    }
}

impl epi::App for EguiView {
    fn name(&self) -> &str {
        "modul"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            instant,
            modul,
            selected_tape,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            ui.heading("modul");

            // ui.add(egui::Slider::u32(age, 0..=120).text("age"));
            ui.label(format!(
                "real time: {:0.1}",
                instant.elapsed().as_secs_f32()
            ));
            ui.label(format!("modul time: {:0.1}", modul.get_audio_index()));
            ui.label(format!("diff: {:0.5}", 0.0));
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    if ui.selectable_label(*selected_tape == 0, "1").clicked() {
                        *selected_tape = 0;
                        modul.set_selected_tape(0);
                    }
                    let time = ui.input().time;

                    let desired_size = ui.available_width() * vec2(0.5, 0.2);
                    let (_id, rect) = ui.allocate_space(desired_size);

                    let to_screen = emath::RectTransform::from_to(
                        Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                        rect,
                    );

                    let mut shapes = vec![];

                    for &mode in &[2, 3, 5] {
                        let mode = mode as f32;
                        let n = 60;
                        let speed = 1.5;

                        let points: Vec<Pos2> = (0..=n)
                            .map(|i| {
                                let t = i as f32 / (n as f32);
                                let amp = (time as f32 * speed * mode).sin() / mode;
                                let y = amp * (t * std::f32::consts::TAU / 2.0 * mode).sin();
                                to_screen * pos2(t, y)
                            })
                            .collect();

                        let thickness = 10.0 / mode;
                        shapes.push(epaint::Shape::line(
                            points,
                            Stroke::new(thickness, Color32::from_additive_luminance(196)),
                        ));
                    }
                    ui.painter().extend(shapes);
                });
                ui.group(|ui| {
                    if ui.selectable_label(*selected_tape == 1, "2").clicked() {
                        *selected_tape = 1;
                        modul.set_selected_tape(1);
                    }
                });
                ui.group(|ui| {
                    if ui.selectable_label(*selected_tape == 2, "3").clicked() {
                        *selected_tape = 2;
                        modul.set_selected_tape(2);
                    }
                });
                ui.group(|ui| {
                    if ui.selectable_label(*selected_tape == 3, "4").clicked() {
                        *selected_tape = 3;
                        modul.set_selected_tape(3);
                    }
                });
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
                                    *selected_tape = 0;
                                    modul.set_selected_tape(0);
                                }
                                Key::Num2 => {
                                    *selected_tape = 1;
                                    modul.set_selected_tape(1);
                                }
                                Key::Num3 => {
                                    *selected_tape = 2;
                                    modul.set_selected_tape(2);
                                }
                                Key::Num4 => {
                                    *selected_tape = 3;
                                    modul.set_selected_tape(3);
                                }
                                Key::R => {
                                    modul.record();
                                }
                                Key::Escape => {
                                    frame.quit();
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
        // frame.set_window_size(ctx.used_size());
        frame.set_window_size(vec2(512.0, 256.0));
    }
}

pub fn start() {
    eframe::run_native(Box::new(EguiView::default()));
}
