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

fn view(_app: &App, model: &Model, _frame: Frame) {
    let r = model.receiver.recv();
    match r {
        Ok(v) => {} //println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    thread::sleep(std::time::Duration::from_millis(33));
}
