use app::Draw;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use nannou_audio::Host;
use std::f64::consts::PI;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

struct Audio {
    phase: f64,
    hz: f64,
    // sender: Sender<Vec<f32>>,
}
struct Model {
    stream: audio::Stream<Audio>,
    audio_host: Host,
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
    let audio = Audio {
        phase: 0.0,
        hz: 440.0,
        // sender,
    };
    let stream = audio_host
        .new_output_stream(audio)
        .render(audio_sine)
        .build()
        .unwrap();
    Model {
        stream,
        audio_host,
        // receiver,
    }
}

// Cache the sine values for better performance
fn audio_sine(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

fn audio_triangle(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let amp = (((audio.phase % 2.0) - 1.0).abs() - 0.5) as f32;
        audio.phase += 10.0 * audio.hz / sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

fn audio_square(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let amp = if audio.phase % 2.0 < 1.0 { -1.0 } else { 1.0 };
        audio.phase += audio.hz / sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

fn audio_saw_tooth(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let amp = (audio.phase % 2.0) as f32 - 1.0;
        audio.phase += audio.hz / sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Q => {
            // let (sender, receiver) = mpsc::channel();
            let audio = Audio {
                phase: 0.0,
                hz: 294.0,
                // sender,
            };
            model.stream = model
                .audio_host
                .new_output_stream(audio)
                .render(audio_sine)
                .build()
                .unwrap();
        }
        Key::W => {}
        Key::E => {}
        Key::R => {}
        Key::Space => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }
        Key::Up => {
            model
                .stream
                .send(|audio| {
                    audio.hz += 10.0;
                })
                .unwrap();
        }
        Key::Down => {
            model
                .stream
                .send(|audio| {
                    audio.hz -= 10.0;
                })
                .unwrap();
        }
        _ => {}
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_sine(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_sine(draw: &Draw) {}
