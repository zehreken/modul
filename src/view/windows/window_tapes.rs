use crate::core::Modul;
use crate::core::{SAMPLE_GRAPH_SIZE, TAPE_COUNT};
use egui::*;

use super::Drawable;

pub struct WindowTapes {
    primary_tape: usize,
    secondary_tapes: [bool; TAPE_COUNT],
    tape_volumes: [f32; TAPE_COUNT],
    tape_mute_states: [bool; TAPE_COUNT],
    tape_solo_states: [bool; TAPE_COUNT],
}

impl Default for WindowTapes {
    fn default() -> Self {
        Self {
            primary_tape: 0,
            secondary_tapes: [false; TAPE_COUNT],
            tape_volumes: [1.0; TAPE_COUNT],
            tape_mute_states: [false; TAPE_COUNT],
            tape_solo_states: [false; TAPE_COUNT],
        }
    }
}

impl Drawable for WindowTapes {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut Modul) {
        let Self {
            primary_tape,
            secondary_tapes,
            tape_volumes,
            tape_mute_states,
            tape_solo_states,
        } = self;

        egui::Window::new("tapes").show(egui_ctx, |ui| {
            egui_ctx.request_repaint();

            ui.colored_label(
                if modul.is_recording_playback() {
                    Color32::RED
                } else {
                    Color32::WHITE
                },
                "MAIN TAPE ⏺",
            );
            for i in 0..TAPE_COUNT {
                draw_tape(
                    ui,
                    primary_tape,
                    secondary_tapes[i],
                    tape_volumes,
                    tape_mute_states,
                    tape_solo_states,
                    modul,
                    i,
                );
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
                                    primary_tape,
                                    0,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num2 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    1,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num3 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    2,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num4 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    3,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num5 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    4,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num6 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    5,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num7 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    6,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::Num8 => {
                                select_tape(
                                    modul,
                                    primary_tape,
                                    7,
                                    secondary_tapes,
                                    *modifiers == Modifiers::SHIFT,
                                );
                            }
                            Key::M => {
                                tape_mute_states[*primary_tape] = !tape_mute_states[*primary_tape];
                                modul.toggle_mute();
                                for i in 0..TAPE_COUNT {
                                    if secondary_tapes[i] {
                                        tape_mute_states[i] = !tape_mute_states[i];
                                    }
                                }
                            }
                            Key::N => {
                                modul.merge_tapes();
                            }
                            Key::S => {
                                tape_solo_states[*primary_tape] = !tape_solo_states[*primary_tape];
                                modul.toggle_solo();
                                for i in 0..TAPE_COUNT {
                                    if secondary_tapes[i] {
                                        tape_solo_states[i] = !tape_solo_states[i];
                                    }
                                }
                            }
                            Key::ArrowUp => {
                                if tape_volumes[*primary_tape] < 1.0 {
                                    tape_volumes[*primary_tape] += 0.01;
                                }
                                for i in 0..TAPE_COUNT {
                                    if secondary_tapes[i] && tape_volumes[i] < 1.0 {
                                        tape_volumes[i] += 0.01;
                                    }
                                }
                                modul.volume_up();
                            }
                            Key::ArrowDown => {
                                if tape_volumes[*primary_tape] > 0.01 {
                                    tape_volumes[*primary_tape] -= 0.01;
                                }
                                for i in 0..TAPE_COUNT {
                                    if secondary_tapes[i] && tape_volumes[i] > 0.01 {
                                        tape_volumes[i] -= 0.01;
                                    }
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

fn select_tape(
    modul: &mut Modul,
    primary_tape: &mut usize,
    tape: usize,
    secondary_tapes: &mut [bool; TAPE_COUNT],
    is_secondary: bool,
) {
    if is_secondary {
        secondary_tapes[tape] = !secondary_tapes[tape];
        modul.select_secondary_tape(tape);
    } else {
        if !modul.is_recording() {
            *primary_tape = tape;
            modul.select_primary_tape(*primary_tape);
        }
    }
}

fn draw_tape(
    ui: &mut Ui,
    primary_tape: &mut usize,
    is_secondary: bool,
    tape_volumes: &mut [f32; TAPE_COUNT],
    tape_mute_states: &mut [bool; TAPE_COUNT],
    tape_solo_states: &mut [bool; TAPE_COUNT],
    modul: &mut Modul,
    id: usize,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            if ui
                .selectable_label(*primary_tape == id, (id + 1).to_string())
                .clicked()
                && !modul.is_recording()
            {
                *primary_tape = id;
                modul.select_primary_tape(id);
            }

            let grayed_out = Color32::from_rgba_unmultiplied(255, 255, 255, 20);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                // ui.group(|ui| {
                ui.colored_label(
                    if *primary_tape == id && modul.is_recording() {
                        Color32::RED
                    } else {
                        grayed_out
                    },
                    "⏺",
                );
                ui.colored_label(
                    if is_secondary {
                        Color32::RED
                    } else {
                        grayed_out
                    },
                    "➕",
                );
                ui.colored_label(
                    if tape_mute_states[id] {
                        Color32::RED
                    } else {
                        grayed_out
                    },
                    "🇲",
                );
                ui.colored_label(
                    if tape_solo_states[id] {
                        Color32::RED
                    } else {
                        grayed_out
                    },
                    "🇸",
                );

                ui.label(format!("{:0.2}", tape_volumes[id]));
                // });
            });
        });

        let desired_size = ui.available_width() * vec2(1.0, 0.03);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

        let mut shapes = vec![];

        // Waveform
        let wavepoints = modul.samples_for_graphs[id];
        let mut index = -1;
        let mut max = 0.0;
        for p in wavepoints {
            if p > max {
                max = p;
            }
        }
        max /= 3.0; // Division is for scaling up
        let points: Vec<Pos2> = wavepoints
            .iter()
            .map(|i| {
                index += 1;
                to_screen * pos2(index as f32 / SAMPLE_GRAPH_SIZE as f32, 1.3 - *i / max)
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
                    Color32::RED
                } else {
                    Color32::GREEN
                },
            ),
        ));
        ui.painter().extend(shapes);
    });
}
