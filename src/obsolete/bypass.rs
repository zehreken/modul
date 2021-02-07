use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer};
use std::sync::mpsc::{Receiver, Sender};

pub struct InModel {
    pub sender: Producer<[f32; 2]>,
}

pub struct OutModel {
    pub receiver: Consumer<[f32; 2]>,
}

pub fn pass_in(model: &mut InModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        model.sender.push([frame[0], frame[1]]).unwrap();
    }
}

pub fn pass_out(model: &mut OutModel, buffer: &mut Buffer) {
    for frame in buffer.frames_mut() {
        let recording_frame = match model.receiver.pop() {
            Some(f) => f,
            None => [0.0, 0.0],
        };
        for (sample, recording_sample) in frame.iter_mut().zip(&recording_frame) {
            *sample = *recording_sample * 0.5;
        }
    }
}
