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
            if modul.is_recording_playback() {
                ui.colored_label(Color32::from_rgb(0, 255, 0), "recording");
            }
            for i in 0..4 {
                group(ui, selected_tape, modul, i);
            }

            // Keyboard input
            for e in ui.input().events.iter() {
                match e {
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                    } => {
                        if *pressed {
                            match key {
                                Key::Num1 => {
                                    if !modul.is_recording() {
                                        *selected_tape = 0;
                                        modul.set_selected_tape(0);
                                    }
                                }
                                Key::Num2 => {
                                    if !modul.is_recording() {
                                        *selected_tape = 1;
                                        modul.set_selected_tape(1);
                                    }
                                }
                                Key::Num3 => {
                                    if !modul.is_recording() {
                                        *selected_tape = 2;
                                        modul.set_selected_tape(2);
                                    }
                                }
                                Key::Num4 => {
                                    if !modul.is_recording() {
                                        *selected_tape = 3;
                                        modul.set_selected_tape(3);
                                    }
                                }
                                Key::R => {
                                    modul.record();
                                }
                                Key::O => {
                                    modul.pause();
                                }
                                Key::P => {
                                    modul.play();
                                }
                                Key::T => {
                                    modul.record_playback();
                                }
                                Key::W => {
                                    modul.write();
                                }
                                Key::C => {
                                    modul.clear();
                                }
                                Key::M => {
                                    modul.mute();
                                }
                                Key::N => {
                                    modul.unmute();
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
        frame.set_window_size(vec2(512.0, 512.0));
    }
}

fn group(ui: &mut Ui, selected_tape: &mut usize, modul: &mut modul::Modul, id: usize) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(*selected_tape == id, (id + 1).to_string())
                .clicked()
            {
                if !modul.is_recording() {
                    *selected_tape = id;
                    modul.set_selected_tape(id);
                }
            }

            if *selected_tape == id && modul.is_recording() {
                ui.colored_label(Color32::from_rgb(255, 0, 0), "recording");
            }
        });
        let time = ui.input().time;

        let desired_size = ui.available_width() * vec2(1.0, 0.1);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

        let mut shapes = vec![];
        // for &mode in &[2, 3, 5] {
        //     let mode = mode as f32;
        //     let n = 30;
        //     let speed = 1.5;

        //     let points: Vec<Pos2> = (0..=n)
        //         .map(|i| {
        //             let t = i as f32 / (n as f32);
        //             let amp = (time as f32 * speed * mode).sin() / mode;
        //             let y = amp * (t * std::f32::consts::TAU / 2.0 * mode).sin();
        //             to_screen * pos2(t, y)
        //         })
        //         .collect();

        //     let thickness = 1.0;
        //     shapes.push(epaint::Shape::line(
        //         points,
        //         Stroke::new(thickness, Color32::from_additive_luminance(196)),
        //     ));
        // }

        let time = modul.get_audio_index() as f32 / super::modul_utils::utils::TAPE_LENGTH as f32;
        let points: Vec<Pos2> = (0..2)
            .map(|i| to_screen * pos2(time, -1.0 + 2.0 * i as f32))
            .collect();
        let thickness = 3.0;
        shapes.push(epaint::Shape::line(
            points,
            Stroke::new(thickness, Color32::from_rgb(255, 0, 0)),
        ));
        ui.painter().extend(shapes);
    });
}

pub fn start() {
    eframe::run_native(Box::new(EguiView::default()));
}
