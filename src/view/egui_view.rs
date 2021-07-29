use super::modul;
use egui::*;
use std::time::Instant;

enum UiState {
    Tapes,
    NewSong,
}

pub struct EguiView {
    ui_state: UiState,
    instant: Instant,
    selected_tape: usize,
    tape_volumes: [f32; 4],
    tape_mute_states: [bool; 4],
}

impl Default for EguiView {
    fn default() -> Self {
        Self {
            ui_state: UiState::Tapes,
            instant: Instant::now(),
            selected_tape: 0,
            tape_volumes: [1.0; 4],
            tape_mute_states: [false; 4],
        }
    }
}

impl EguiView {
    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        match self.ui_state {
            UiState::NewSong => self.draw_new_song(ctx, modul),
            UiState::Tapes => self.show_tapes(ctx, modul),
        }
    }

    fn draw_new_song(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        let mut bpm = 120.0;
        let mut bar_count = 4;
        egui::Window::new("new song").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("bpm: {}", bpm));
                if ui.small_button("-").clicked() {
                    bpm -= 1.0;
                }
                if ui.small_button("+").clicked() {
                    bpm += 1.0;
                }
            });
            ui.horizontal(|ui| {
                ui.label(format!("bar count: {}", bar_count));
                if ui.small_button("-").clicked() {
                    bar_count -= 1;
                }
                if ui.small_button("+").clicked() {
                    bar_count += 1;
                }
            });
            if ui.button("Create").clicked() {
                // modul.new_song(bpm, bar_count);
            }
        });
    }

    fn show_tapes(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        let Self {
            ui_state,
            instant,
            selected_tape,
            tape_volumes,
            tape_mute_states,
        } = self;

        egui::Window::new("modul").show(ctx, |ui| {
            ctx.request_repaint();
            ui.heading("modul");

            ui.label(format!(
                "real time: {:0.1} sec",
                instant.elapsed().as_secs_f32()
            ));
            ui.label(format!("modul time: {:0.1}", modul.get_audio_index()));
            // ui.label(format!("diff: {:0.5}", 0.0));
            ui.label(format!("bar length: {} sec", 8.0));

            if modul.is_recording_playback() {
                ui.colored_label(Color32::from_rgb(0, 255, 0), "recording");
            }
            for i in 0..4 {
                draw_tape(ui, selected_tape, tape_volumes, tape_mute_states, modul, i);
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
                                    tape_mute_states[*selected_tape] =
                                        !tape_mute_states[*selected_tape];
                                    if tape_mute_states[*selected_tape] {
                                        modul.mute();
                                    } else {
                                        modul.unmute();
                                    }
                                }
                                Key::N => {
                                    // Use this to create new song
                                }
                                Key::ArrowUp => {
                                    if tape_volumes[*selected_tape] < 1.0 {
                                        tape_volumes[*selected_tape] += 0.05;
                                    }
                                    modul.volume_up();
                                }
                                Key::ArrowDown => {
                                    if tape_volumes[*selected_tape] > 0.0 {
                                        tape_volumes[*selected_tape] -= 0.05;
                                    }
                                    modul.volume_down();
                                }
                                Key::Escape => {
                                    std::process::exit(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

fn draw_tape(
    ui: &mut Ui,
    selected_tape: &mut usize,
    tape_volumes: &mut [f32; 4],
    tape_mute_states: &mut [bool; 4],
    modul: &mut modul::Modul,
    id: usize,
) {
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

            ui.label(format!("{:0.2}", tape_volumes[id]));

            if *selected_tape == id && modul.is_recording() {
                ui.colored_label(Color32::from_rgb(255, 0, 0), "recording");
            }
        });

        let desired_size = ui.available_width() * vec2(1.0, 0.02);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

        let mut shapes = vec![];

        let time = modul.get_audio_index() as f32 / modul.tape_length as f32;
        let points: Vec<Pos2> = (0..2)
            .map(|i| to_screen * pos2(time, -1.0 + 2.0 * i as f32))
            .collect();
        shapes.push(epaint::Shape::line(
            points,
            Stroke::new(
                3.0,
                if tape_mute_states[id] {
                    Color32::from_rgb(255, 0, 0)
                } else {
                    Color32::from_rgb(0, 255, 0)
                },
            ),
        ));
        ui.painter().extend(shapes);
    });
}
