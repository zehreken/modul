use super::envelope::*;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Host;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};

use super::wave::*;

struct Model {
    stream: audio::Stream<AudioE>,
    audio_host: Host,
    freqDivider: f64,
    // receiver: Receiver<Vec<f32>>,
}

pub fn run_modul() {
    nannou::app(model).event(event).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 512)
        .title("modul")
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    // let (sender, receiver) = mpsc::channel();
    // Initialize the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialize the state that we want to live on the audio thread.
    // let audio = Audio {
    //     phase: 0.0,
    //     hz: 440.0,
    //     // sender,
    // };
    let envelopes = vec![];
    let model = AudioE { envelopes };
    let stream = audio_host
        .new_output_stream(model)
        .render(audioE)
        .build()
        .unwrap();
    Model {
        stream,
        audio_host,
        freqDivider: 1.0,
        // receiver,
    }
}

fn record(model: &mut Model) {
    let audio = Audio {
        phase: 0.0,
        hz: 440.0,
    };
    let input_stream = model
        .audio_host
        .new_input_stream(audio)
        .capture(_capture)
        .build()
        .unwrap();
}

fn _capture(audio: &mut Audio, buffer: &nannou_audio::Buffer) {

}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Q => {
            create_sine_stream(model, 0);
        }
        Key::W => {
            create_sine_stream(model, 1);
        }
        Key::E => {
            create_sine_stream(model, 2);
        }
        Key::R => {
            create_sine_stream(model, 3);
        }
        Key::A => {
            create_sine_stream(model, 4);
        }
        Key::S => {
            create_sine_stream(model, 5);
        }
        Key::D => {
            create_sine_stream(model, 6);
        }
        Key::F => {
            create_sine_stream(model, 7);
        }
        Key::T => {
            record(model);
        }
        Key::Space => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }
        Key::Up => {
            // model
            //     .stream
            //     .send(|audio| {
            //         audio.hz += 10.0;
            //     })
            //     .unwrap();
            model.freqDivider += 1.0;
        }
        Key::Down => {
            // model
            //     .stream
            //     .send(|audio| {
            //         audio.hz -= 10.0;
            //     })
            //     .unwrap();
            if model.freqDivider > 1.0 {
                model.freqDivider -= 1.0;
            }
        }
        _ => {}
    }
}

fn create_sine_stream(model: &mut Model, key: usize) {
    let audio = get_audio_model(key);

    // model.stream = model
    //     .audio_host
    //     .new_output_stream(audio)
    //     .render(audio_sine)
    //     .build()
    //     .unwrap();
    let env = Envelope {
        start: std::time::Instant::now(),
        duration: 1.0,
        phase: audio.phase,
        hz: audio.hz / model.freqDivider,
    };
    model
        .stream
        .send(move |audio| {
            audio.envelopes.push(env);
        })
        .ok();
}

fn create_square_stream(model: &mut Model, key: usize) {
    let audio = get_audio_model(key);

    // model.stream = model
    //     .audio_host
    //     .new_output_stream(audio)
    //     .render(audio_square)
    //     .build()
    //     .unwrap();
}

fn get_audio_model(key: usize) -> Audio {
    let keys = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ];
    Audio {
        phase: 0.0,
        hz: keys[key % 8],
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_sine(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_sine(_draw: &Draw) {}
