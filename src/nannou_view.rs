use crate::{modul_utils, tape::tape_view::TapeView};

use super::modul;
use nannou::prelude::*;
use std::thread;
use std::time::Instant;
struct Model {
    instant: Instant,
    tape_views: [TapeView; 4],
    modul: modul::Modul,
}

impl Model {
    fn set_selected_tape(&mut self, selected_tape: usize) {
        for i in 0..self.tape_views.len() {
            self.tape_views[i].is_selected = i == selected_tape;
        }
    }
}

pub fn start() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 512)
        .title("modul")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let modul = modul::Modul::new();

    let tape_views = [
        TapeView::new(-300.0, 0.0),
        TapeView::new(-100.0, 0.0),
        TapeView::new(100.0, 0.0),
        TapeView::new(300.0, 0.0),
    ];

    Model {
        instant: Instant::now(),
        tape_views,
        modul,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.modul.set_selected_tape(0);
            model.set_selected_tape(0);
        }
        Key::Key2 => {
            model.modul.set_selected_tape(1);
            model.set_selected_tape(1);
        }
        Key::Key3 => {
            model.modul.set_selected_tape(2);
            model.set_selected_tape(2);
        }
        Key::Key4 => {
            model.modul.set_selected_tape(3);
            model.set_selected_tape(3);
        }
        Key::R => {
            model.modul.record();
        }
        Key::P => {
            model.modul.play();
        }
        Key::W => {
            model.modul.write();
        }
        Key::C => {
            model.modul.clear_all();
        }
        Key::M => {
            model.modul.mute();
        }
        Key::N => {
            model.modul.unmute();
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let elapsed_secs = model.instant.elapsed().as_secs_f32();
    draw.text(&format!(
        "real time: {:0.1}\nmodul time {:0.1}\ndiff {:0.5}",
        elapsed_secs,
        model.modul.get_time(),
        elapsed_secs - model.modul.get_time(),
    ))
    .font_size(20)
    .x_y(0.0, 200.0)
    .color(YELLOW);

    let x = model.modul.get_audio_index() / 44100;
    draw.rect()
        .w_h(100.0, 20.0)
        .x_y(x as f32 * 100.0 - 350.0, 100.0)
        .color(CRIMSON);

    let cursor_position =
        model.modul.get_audio_index() as f32 / modul_utils::utils::TAPE_LENGTH as f32;
    for view in model.tape_views.iter() {
        view.draw(&draw, cursor_position)
    }

    draw.to_frame(app, &frame).unwrap();

    thread::sleep(std::time::Duration::from_millis(33));
}
