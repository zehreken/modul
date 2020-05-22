use nannou_audio::Buffer;
use std::sync::mpsc::Sender;

pub struct RecordModel {
    pub sender: Sender<Vec<f32>>,
}

pub struct RecordingAudio {
    pub recordings: Vec<Vec<f32>>,
}

pub fn capture(model: &mut RecordModel, buffer: &Buffer) {
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push(frame[0]);
    }
    model.sender.send(frames).unwrap();
}

pub fn audio(audio: &mut RecordingAudio, buffer: &mut Buffer) {
    let mut have_ended = vec![];
    let len_frames = 0;

    for (i, recording) in audio.recordings.iter_mut().enumerate() {
        let mut frame_count = 0;
        let recording_copy = recording.clone();
        for (frame, file_frame) in buffer.frames_mut().zip(recording_copy) {
            for sample in frame.iter_mut() {
                *sample = file_frame;
            }
            frame_count += 1;
        }

        for i in (0..frame_count).rev() {
            recording.remove(i);
        }

        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    // Remove all sounds that have ended
    for i in have_ended.into_iter().rev() {
        audio.recordings.remove(i);
    }
}
