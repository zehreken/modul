use nannou_audio::Buffer;
use std::sync::mpsc::{Receiver, Sender};

pub struct InModel {
    pub sender: Sender<Vec<[f32; 2]>>,
}

pub struct OutModel {
    pub receiver: Receiver<Vec<[f32; 2]>>,
}

pub fn pass_in(model: &mut InModel, buffer: &Buffer) {
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push([frame[0], frame[1]]);
    }
    model.sender.send(frames).unwrap();
}

pub fn pass_out(model: &mut OutModel, buffer: &mut Buffer) {
    let mut recording = vec![];
    for r in model.receiver.try_iter() {
        recording = r;
    }
    for (frame, recording_frame) in buffer.frames_mut().zip(recording) {
        for (sample, recording_sample) in frame.iter_mut().zip(&recording_frame) {
            *sample = *recording_sample * 0.5;
        }
    }
}
