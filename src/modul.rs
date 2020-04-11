use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

struct Audio {
    phase: f64,
    hz: f64,
    sign: f64,
}
struct Model {
    stream: audio::Stream<Audio>,
}

pub fn run_modul() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    // Initialize the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialize the state that we want to live on the audio thread.
    let model = Audio {
        phase: 0.0,
        hz: 440.0,
        sign: 1.0,
    };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio_saw_tooth)
        .build()
        .unwrap();
    Model { stream }
}

// Cache the sine values for better performance
fn audio_sine(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        let sine_amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn audio_triangle(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        audio.phase += 0.01 * audio.sign;
        if audio.phase >= 1.0 {
            audio.sign = -1.0;
        } else if audio.phase <= -1.0 {
            audio.sign = 1.0;
        }
        for channel in frame {
            *channel = audio.phase as f32 * volume;
        }
    }
}

fn audio_square(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        if audio.sign < 120.0 {
            audio.phase = -1.0;
        } else if audio.sign < 240.0 {
            audio.phase = 1.0;
        }
        audio.sign += 1.0;
        if audio.sign >= 240.0 {
            audio.sign = 0.0;
        }
        for channel in frame {
            *channel = audio.phase as f32 * volume;
        }
    }
}

fn audio_saw_tooth(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    for frame in buffer.frames_mut() {
        audio.phase += 0.01;
        audio.sign += 1.0;
        if audio.sign >= 100.0 {
            audio.phase = 0.0;
            audio.sign = 0.0;
        }
        for channel in frame {
            *channel = audio.phase as f32 * volume;
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
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

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}
