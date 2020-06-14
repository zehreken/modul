use super::beat_controller::BeatController;
use super::envelope::*;
use super::graphics::*;
use super::record::*;
use super::traits::Nannou;
use super::wave::*;
use hound;
use nannou::prelude::*;
use nannou_audio as audio;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub const SAMPLE_RATE: usize = 44100;
pub const TAPE_SECONDS: usize = 1;
pub const TAPE_SAMPLES: usize = SAMPLE_RATE * TAPE_SECONDS;

struct Model {
    global_time: u32,
    wave_stream: audio::Stream<WaveModel>,
    tape_stream: audio::Stream<TapeModel>,
    input_stream: audio::Stream<InputModel>,
    freq_divider: f64,
    time_receiver: Receiver<u32>,
    receiver: Receiver<Vec<[f32; 2]>>,
    temp_tape: Vec<[f32; 2]>,
    beat_controller: BeatController,
    selected_tape: u8,
    tape_graphs: Vec<Tape>,
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

    let (sender, receiver) = mpsc::channel();
    let (time_sender, time_receiver) = mpsc::channel();
    // Initialize the audio API so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialize the state that we want to live on the audio thread.
    let wave_model = WaveModel { envelopes: vec![] };
    let wave_stream = audio_host
        .new_output_stream(wave_model)
        .render(audio_wave)
        .build()
        .unwrap();
    wave_stream.pause().unwrap();

    // A tape is 4 seconds long
    let tapes = vec![vec![[0.0; 2]; TAPE_SAMPLES]; 4];
    let tape_model = TapeModel {
        time_sender,
        index: 0,
        volume: 0.5,
        tapes,
    };
    let tape_stream = audio_host
        .new_output_stream(tape_model)
        .render(playback_tape)
        .build()
        .unwrap();

    let capture_model = InputModel { sender };
    let capture_stream = audio_host
        .new_input_stream(capture_model)
        .capture(capture)
        .build()
        .unwrap();
    capture_stream.pause().unwrap();

    let mut tape_graphs = vec![];
    for i in 0..4 {
        tape_graphs.push(Tape {
            pos_x: -384.0 + i as f32 * 256.0,
            pos_y: 0.0,
            is_selected: i == 0,
        });
    }

    Model {
        global_time: 0,
        wave_stream,
        tape_stream,
        input_stream: capture_stream,
        freq_divider: 1.0,
        time_receiver,
        receiver,
        temp_tape: vec![],
        beat_controller: BeatController::new(120, 4, 1),
        selected_tape: 0,
        tape_graphs,
    }
}

fn record(model: &mut Model) {
    if model.input_stream.is_paused() {
        model.temp_tape.clear();
        model.input_stream.play().unwrap();
    } else {
        model.input_stream.pause().unwrap();
        let selected_tape = model.selected_tape as usize;
        for i in 0..TAPE_SAMPLES {
            let mut frame = [0.0, 0.0];
            if i < model.temp_tape.len() {
                frame = model.temp_tape[i];
            }
            model
                .tape_stream
                .send(move |audio| {
                    audio.tapes[selected_tape][i] = frame;
                })
                .unwrap();
        }
    }
    println!("record start {}", model.input_stream.is_playing());
}

fn play(model: &mut Model) {
    // if model.playback_stream.is_paused() {
    //     clear_recordings(model);
    //     fill_playback_stream(model);
    //     model.playback_stream.play().unwrap();
    // } else {
    //     model.playback_stream.pause().unwrap();
    // }
    // println!("play start {}", model.playback_stream.is_playing());
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            select_tape(0, model);
        }
        Key::Key2 => {
            select_tape(1, model);
        }
        Key::Key3 => {
            select_tape(2, model);
        }
        Key::Key4 => {
            select_tape(3, model);
        }
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
        Key::G => {
            play(model);
        }
        Key::Y => {
            write(model);
        }
        Key::M => {
            toggle_volume(model);
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

fn toggle_volume(model: &Model) {
    model
        .tape_stream
        .send(|audio| {
            if audio.volume > 0.0 {
                audio.volume = 0.0;
            } else {
                audio.volume = 0.5;
            }
        })
        .unwrap();
}

fn write(model: &Model) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    println!("writing: {}", model.temp_tape.len());
    let mut writer = hound::WavWriter::create("recording.wav", spec).unwrap();
    for frame in model.temp_tape.iter() {
        let sample = frame[1];
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn create_sine_stream(model: &Model, key: usize) {
    let tone = get_key(key);

    // model.stream = model
    //     .audio_host
    //     .new_output_stream(audio)
    //     .render(audio_sine)
    //     .build()
    //     .unwrap();
    let env = Envelope {
        start: std::time::Instant::now(),
        duration: 1000,
        phase: 0.0,
        hz: tone / model.freq_divider,
        frame: 0,
    };
    model
        .wave_stream
        .send(move |audio| {
            audio.envelopes.push(env);
        })
        .ok();
}

fn get_key(key: usize) -> f64 {
    // C, D, E, F, G, A, B, C
    let keys = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ];
    keys[key % 8]
}

fn select_tape(index: u8, model: &mut Model) {
    model.selected_tape = index;
    for (i, tape) in model.tape_graphs.iter_mut().enumerate() {
        tape.is_selected = i == index as usize;
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for frames in model.receiver.try_iter() {
        for i in frames {
            model.temp_tape.push(i);
        }
    }

    let previous_time = model.global_time;
    for t in model.time_receiver.try_iter() {
        model.global_time = t;
    }
    let mut delta_time = model.global_time as i32 - previous_time as i32;
    if delta_time < 0 {
        delta_time += SAMPLE_RATE as i32;
    }

    // println!("diff: {}", delta_time);
    // delta_time = (delta_time as f32 / SAMPLE_RATE as f32 * 1000.0) as i32;

    model.beat_controller.update(delta_time);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for tape in model.tape_graphs.iter() {
        tape.draw(&draw, &model.beat_controller);
    }
    model.beat_controller.draw(&draw);

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
