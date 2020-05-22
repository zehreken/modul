use nannou_audio::Buffer;
use std::sync::mpsc::Sender;

pub struct RecordModel {
    pub sender: Sender<Vec<f32>>,
}

pub fn capture(model: &mut RecordModel, buffer: &Buffer) {
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push(frame[0]);
    }
    model.sender.send(frames).unwrap();
}
