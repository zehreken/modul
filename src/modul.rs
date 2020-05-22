use super::envelope::*;
use super::record::*;
use super::wave::*;
use nannou::prelude::*;
use nannou_audio as audio;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

struct Model {
    wave_stream: audio::Stream<WaveModel>,
    playback_stream: audio::Stream<PlaybackModel>,
    capture_stream: audio::Stream<CaptureModel>,
    freq_divider: f64,
    receiver: Receiver<Vec<f32>>,
    recording: Vec<f32>,
}

pub fn run_modul() {
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

    let (sender, receiver) = mpsc::channel();
    // Initialize the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialize the state that we want to live on the audio thread.
    let wave_model = WaveModel { envelopes: vec![] };
    let wave_stream = audio_host
        .new_output_stream(wave_model)
        .render(audioE)
        .build()
        .unwrap();

    let playback_model = PlaybackModel { recordings: vec![] };
    let playback_stream = audio_host
        .new_output_stream(playback_model)
        .render(playback)
        .build()
        .unwrap();

    let capture_model = CaptureModel { sender };
    let capture_stream = audio_host
        .new_input_stream(capture_model)
        .capture(capture)
        .build()
        .unwrap();

    Model {
        wave_stream,
        playback_stream,
        capture_stream,
        freq_divider: 1.0,
        receiver,
        recording: vec![],
    }
}

fn record(model: &mut Model) {
    if model.capture_stream.is_playing() {
        println!("play");
        model.capture_stream.pause().unwrap();
        clear_recordings(model);
        create_playback_stream(model);
        model.playback_stream.play().unwrap();
    } else {
        println!("record");
        model.playback_stream.pause().unwrap();
        model.capture_stream.play().unwrap();
        model.recording.clear();
    }
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
            if model.wave_stream.is_playing() {
                model.wave_stream.pause().unwrap();
            } else {
                model.wave_stream.play().unwrap();
            }
        }
        Key::Up => {
            // model
            //     .stream
            //     .send(|audio| {
            //         audio.hz += 10.0;
            //     })
            //     .unwrap();
            model.freq_divider += 1.0;
        }
        Key::Down => {
            // model
            //     .stream
            //     .send(|audio| {
            //         audio.hz -= 10.0;
            //     })
            //     .unwrap();
            if model.freq_divider > 1.0 {
                model.freq_divider -= 1.0;
            }
        }
        _ => {}
    }
}

fn create_sine_stream(model: &mut Model, key: usize) {
    let tone = get_key(key);

    // model.stream = model
    //     .audio_host
    //     .new_output_stream(audio)
    //     .render(audio_sine)
    //     .build()
    //     .unwrap();
    let env = Envelope {
        start: std::time::Instant::now(),
        duration: 1.0,
        phase: 0.0,
        hz: tone / model.freq_divider,
    };
    model
        .wave_stream
        .send(move |audio| {
            audio.envelopes.push(env);
        })
        .ok();
}

fn create_playback_stream(model: &mut Model) {
    let r = model.recording.clone();
    println!("{}", r.len());
    model
        .playback_stream
        .send(move |audio| {
            audio.recordings.push(r);
        })
        .ok();
}

fn clear_recordings(model: &mut Model) {
    model
        .playback_stream
        .send(|audio| {
            audio.recordings.clear();
        })
        .ok();
}

fn get_key(key: usize) -> f64 {
    let keys = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ];
    keys[key % 8]
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for frames in model.receiver.try_iter() {
        for i in frames {
            model.recording.push(i);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    /*
    let scale = 1000.0;
    let mut frames: Vec<f32> = vec![];
    for f in model.receiver.try_iter() {
        frames = f;
        for i in frames {
            model.recording.push(i);
        }
    }
    let mut index = -128.0;
    let points = frames.iter().map(|i| {
        let x = index;
        let y = *i;
        index += 4.0;
        pt2(x, y * scale)
    });

    draw.polyline().points(points).color(GOLD);
    */

    draw_sine(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_sine(_draw: &Draw) {}
