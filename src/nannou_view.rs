use super::modul_cpal;
use nannou::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
struct Model {
    receiver: Receiver<f32>,
    key_sender: Sender<u8>,
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

    let (sender, receiver) = channel();
    let (key_sender, key_receiver) = channel();

    thread::spawn(move || {
        modul_cpal::start(sender, key_receiver);
    });
    Model {
        receiver,
        key_sender,
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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let r = model.receiver.recv();
    match r {
        Ok(v) => {
            draw.text(&format!("time {:0.1}", v))
                .font_size(50)
                .x_y(0.0, 0.0)
                .color(YELLOW);
        }
        Err(e) => println!("{}", e),
    }

    draw.to_frame(app, &frame).unwrap();

    thread::sleep(std::time::Duration::from_millis(33));
}
