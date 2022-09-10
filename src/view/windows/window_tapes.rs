use crate::modul::Modul;
use crate::modul_utils::utils::{SAMPLE_GRAPH_SIZE, TAPE_COUNT};
use egui::*;

use super::Drawable;

pub struct WindowTapes {
    selected_tape: usize,
    tape_volumes: [f32; TAPE_COUNT],
    tape_mute_states: [bool; TAPE_COUNT],
}

impl Default for WindowTapes {
    fn default() -> Self {
        Self {
            selected_tape: 0,
            tape_volumes: [1.0; TAPE_COUNT],
            tape_mute_states: [false; TAPE_COUNT],
        }
    }
}

impl Drawable for WindowTapes {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut Modul) {
        let Self {
            selected_tape,
            tape_volumes,
            tape_mute_states,
        } = self;

        egui::Window::new("tapes").show(egui_ctx, |ui| {
            egui_ctx.request_repaint();
            ui.heading("tapes");

            if modul.is_recording_playback() {
                ui.colored_label(Color32::from_rgb(0, 255, 0), "recording");
            }
            for i in 0..TAPE_COUNT {
                draw_tape(ui, selected_tape, tape_volumes, tape_mute_states, modul, i);
            }

            // Keyboard input
            for e in ui.input().events.iter() {
                if let egui::Event::Key {
                    key,
                    pressed,
                    modifiers,
                } = e
                {
                    if *pressed {
                        match key {
                            Key::Num1 => {
                                select_tape(
                                    modul,
                                    selected_tape,
                                    0,
                                    *modifiers == Modifiers::SHIFT,
                                );
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
                            Key::Num5 => {
                                if !modul.is_recording() {
                                    *selected_tape = 4;
                                    modul.set_selected_tape(4);
                                }
                            }
                            Key::Num6 => {
                                if !modul.is_recording() {
                                    *selected_tape = 5;
                                    modul.set_selected_tape(5);
                                }
                            }
                            Key::Num7 => {
                                if !modul.is_recording() {
                                    *selected_tape = 6;
                                    modul.set_selected_tape(6);
                                }
                            }
                            Key::Num8 => {
                                if !modul.is_recording() {
                                    *selected_tape = 7;
                                    modul.set_selected_tape(7);
                                }
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
                            _ => {}
                        }
                    }
                }
            }
        });
    }
}

fn select_tape(modul: &mut Modul, selected_tape: &mut usize, tape: usize, is_secondary: bool) {
    if is_secondary {
    } else {
        if !modul.is_recording() {
            *selected_tape = tape;
            modul.set_selected_tape(*selected_tape);
        }
    }
}

fn draw_tape(
    ui: &mut Ui,
    selected_tape: &mut usize,
    tape_volumes: &mut [f32; TAPE_COUNT],
    tape_mute_states: &mut [bool; TAPE_COUNT],
    modul: &mut Modul,
    id: usize,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(*selected_tape == id, (id + 1).to_string())
                .clicked()
                && !modul.is_recording()
            {
                *selected_tape = id;
                modul.set_selected_tape(id);
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

        // Waveform
        let wavepoints = modul.samples_for_graphs[id];
        let mut index = -1;
        let points: Vec<Pos2> = wavepoints
            .iter()
            .map(|i| {
                index += 1;
                to_screen * pos2(index as f32 / SAMPLE_GRAPH_SIZE as f32, -*i * 0.02)
            })
            .collect();

        shapes.push(epaint::Shape::line(
            points,
            Stroke::new(1.0, Color32::WHITE),
        ));
        // Waveform

        // Second waveform
        // https://github.com/not-fl3/egui-miniquad/issues/42
        // index = -1;
        // let points_neg: Vec<Pos2> = wavepoints
        //     .iter()
        //     .map(|i| {
        //         index += 1;
        //         to_screen * pos2(index as f32 / SAMPLE_GRAPH_SIZE as f32, *i * 0.02)
        //     })
        //     .collect();

        // shapes.push(epaint::Shape::line(
        //     points_neg,
        //     Stroke::new(1.0, Color32::WHITE),
        // ));
        // Second waveform

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
