use super::modul_cpal;
use nannou::prelude::*;
use std::thread;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    time::Instant,
};
struct Model {
    time_receiver: Receiver<f32>,
    key_sender: Sender<u8>,
    instant: Instant,
    buffer_time: f32,
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

    let (time_sender, time_receiver) = channel();
    let (key_sender, key_receiver) = channel();

    thread::spawn(move || {
        modul_cpal::start(time_sender, key_receiver);
    });
    Model {
        time_receiver,
        key_sender,
        instant: Instant::now(),
        buffer_time: 0.0,
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.key_sender.send(1).unwrap();
        }
        Key::Key2 => {
            model.key_sender.send(2).unwrap();
        }
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for v in model.time_receiver.try_iter() {
        model.buffer_time += v;
    }
    // let r = model.receiver.recv();
    // match r {
    //     Ok(v) => model.buffer_time += v,
    //     Err(e) => println!("{}", e),
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let elapsed_secs = model.instant.elapsed().as_secs_f32();
    draw.text(&format!(
        "real time: {:0.1}\nbuffer time {:0.1}\ndiff {:0.5}",
        elapsed_secs,
        model.buffer_time,
        elapsed_secs - model.buffer_time,
    ))
    .font_size(50)
    .x_y(0.0, 0.0)
    .color(YELLOW);

    draw.to_frame(app, &frame).unwrap();

    thread::sleep(std::time::Duration::from_millis(33));
}
