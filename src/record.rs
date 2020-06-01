use nannou_audio::Buffer;
use std::sync::mpsc::Sender;

pub struct CaptureModel {
    pub sender: Sender<Vec<[f32; 2]>>,
}

pub struct PlaybackModel {
    pub recordings: Vec<Vec<[f32; 2]>>,
}

pub fn capture(model: &mut CaptureModel, buffer: &Buffer) {
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames() {
        frames.push([frame[0], frame[1]]);
    }
    model.sender.send(frames).unwrap();
}

pub fn playback(audio: &mut PlaybackModel, buffer: &mut Buffer) {
    // let mut have_ended = vec![];
    // let len_frames = 0;

    for (i, recording) in audio.recordings.iter_mut().enumerate() {
        let mut frame_count = 0;
        for (frame, file_frame) in buffer.frames_mut().zip(recording.iter()) {
            for (sample, frame_sample) in frame.iter_mut().zip(file_frame) {
                *sample = *frame_sample;
            }
            frame_count += 1;
        }

        for i in (0..frame_count).rev() {
            recording.remove(i);
        }

        // if frame_count < len_frames {
        //     have_ended.push(i);
        // }
    }

    // Remove all sounds that have ended
    // for i in have_ended.into_iter().rev() {
    //     audio.recordings.remove(i);
    // }
}
