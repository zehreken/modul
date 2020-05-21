use super::envelope::*;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Host;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use super::wave::*;

struct Model {
    stream: audio::Stream<AudioE>,
    recording_stream: audio::Stream<RecordingAudio>,
    input_stream: audio::Stream<Audio>,
    audio_host: Host,
    freqDivider: f64,
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

    let recording_model = RecordingAudio { recordings: vec![] };
    let recording_stream = audio_host
        .new_output_stream(recording_model)
        .render(audio)
        .build()
        .unwrap();
    let audio = Audio {
        phase: 0.0,
        hz: 440.0,
        sender,
    };

    let input_stream = audio_host
        .new_input_stream(audio)
        .capture(_capture)
        .build()
        .unwrap();

    // This blocks the receiver
    // input_stream.pause().unwrap();
    Model {
        stream,
        recording_stream,
        input_stream,
        audio_host,
        freqDivider: 1.0,
        receiver,
        recording: vec![],
    }
}

fn record(model: &mut Model) {
    if model.input_stream.is_playing() {
        println!("play");
        model.input_stream.pause().unwrap();
        clear_recordings(model);
        create_playback_stream(model);
        model.recording_stream.play().unwrap();
    } else {
        println!("record");
        model.recording_stream.pause().unwrap();
        model.input_stream.play().unwrap();
        model.recording.clear();
    }
}

fn _capture(audio: &mut Audio, buffer: &nannou_audio::Buffer) {
    // println!("{:?}", buffer.frames().last());
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push(frame[1]);
    }
    audio.sender.send(frames).unwrap();
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
    let tone = get_audio_model(key);

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
        hz: tone / model.freqDivider,
    };
    model
        .stream
        .send(move |audio| {
            audio.envelopes.push(env);
        })
        .ok();
}

fn create_playback_stream(model: &mut Model) {
    let r = model.recording.clone();
    println!("{}", r.len());
    model
        .recording_stream
        .send(move |audio| {
            audio.recordings.push(r);
        })
        .ok();
}

fn clear_recordings(model: &mut Model) {
    model
        .recording_stream
        .send(|audio| {
            audio.recordings.clear();
        })
        .ok();
}

fn audio(audio: &mut RecordingAudio, buffer: &mut nannou_audio::Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    // Sum all of the sounds onto the buffer.
    // println!("{}", audio.recordings.len());
    for (i, recording) in audio.recordings.iter_mut().enumerate() {
        let mut frame_count = 0;
        // let file_frames = recording.frames::<[f32; 2]>().filter_map(Result::ok);
        let recording_copy = recording.clone();
        for (frame, file_frame) in buffer.frames_mut().zip(recording_copy) {
            for sample in frame.iter_mut() {
                *sample = file_frame;
            }
            frame_count += 1;
        }

        if recording.len() < frame_count {
            frame_count = recording.len();
        }
        for i in (0..frame_count).rev() {
            recording.remove(i);
        }

        // If the sound yielded less samples than are in the buffer, it must have ended.
        // if frame_count < len_frames {
        //     have_ended.push(i);
        // }
    }

    // Remove all sounds that have ended.
    for i in have_ended.into_iter().rev() {
        audio.recordings.remove(i);
    }
}

fn get_audio_model(key: usize) -> f64 {
    let keys = [
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ];
    // Audio {
    //     phase: 0.0,
    //     hz: keys[key % 8],
    // }
    keys[key % 8]
}

fn update(app: &App, model: &mut Model, _update: Update) {
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
