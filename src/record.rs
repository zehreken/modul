use nannou_audio::Buffer;
use std::sync::mpsc::Sender;

pub struct TapeModel {
    pub time_sender: Sender<u32>,
    pub index: usize,
    pub volume: f32,
    pub tapes: Vec<Vec<[f32; 2]>>,
}

pub struct InputModel {
    pub sender: Sender<Vec<[f32; 2]>>,
}

pub fn capture(model: &mut InputModel, buffer: &Buffer) {
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push([frame[0], frame[1]]);
    }
    model.sender.send(frames).unwrap();
}

pub fn playback_tape(audio: &mut TapeModel, buffer: &mut Buffer) {
    for frame in buffer.frames_mut() {
        for (i, tape) in audio.tapes.iter_mut().enumerate() {
            let tape_frame = tape[audio.index];
            for (sample, tape_sample) in frame.iter_mut().zip(&tape_frame) {
                *sample += *tape_sample * audio.volume;
            }
        }

        audio.index += 1;
        // 44100 samples equal to 1 second
        if audio.index == super::modul::TAPE_SAMPLES {
            // println!("{} seconds", super::modul::TAPE_SECONDS);
            audio.index = 0;
        }

        let time = audio.index % super::modul::SAMPLE_RATE;
        audio.time_sender.send(time as u32).unwrap();
    }
}
