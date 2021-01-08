use super::modul_second;
use nannou::prelude::*;
use std::thread;
use std::time::Instant;
struct Model {
    instant: Instant,
    modul: modul_second::Modul,
}

pub fn start() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 512)
        .title("modul")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let modul = modul_second::Modul::new();

    Model {
        instant: Instant::now(),
        modul,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.modul.record();
        }
        Key::Key2 => {
            model.modul.play();
        }
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.modul.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let elapsed_secs = model.instant.elapsed().as_secs_f32();
    draw.text(&format!(
        "real time: {:0.1}\nmodul    time {:0.1}\ndiff {:0.5}",
        elapsed_secs,
        model.modul.get_time(),
        elapsed_secs - model.modul.get_time(),
    ))
    .font_size(50)
    .x_y(0.0, 0.0)
    .color(YELLOW);

    draw.to_frame(app, &frame).unwrap();

    thread::sleep(std::time::Duration::from_millis(33));
}
