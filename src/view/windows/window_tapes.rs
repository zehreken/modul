use crate::modul::Modul;
use crate::modul_utils::utils::TAPE_COUNT;
use egui::*;

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

impl WindowTapes {
    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut Modul) {
        let Self {
            selected_tape,
            tape_volumes,
            tape_mute_states,
        } = self;

        egui::Window::new("tapes").show(ctx, |ui| {
            ctx.request_repaint();
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
                    modifiers: _,
                } = e
                {
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
                            Key::Y => {
                                modul.play_through();
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
            }
        });
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
