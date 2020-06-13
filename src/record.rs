use nannou_audio::Buffer;
use std::sync::mpsc::Sender;

pub struct TapeModel {
    pub time_sender: Sender<u32>,
    pub index: usize,
    pub volume: f32,
    pub tapes: Vec<Vec<[f32; 2]>>,
}

pub struct CaptureModel {
    pub sender: Sender<Vec<[f32; 2]>>,
}

pub struct PlaybackModel {
    pub index: usize,
    pub recordings: Vec<Vec<[f32; 2]>>,
}

pub fn capture(model: &mut CaptureModel, buffer: &Buffer) {
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
        if audio.index == 44100 {
            // println!("1 second");
            audio.index = 0;
        }
        audio.time_sender.send(audio.index as u32).unwrap();
    }
}

pub fn playback(audio: &mut PlaybackModel, buffer: &mut Buffer) {
    let mut have_ended = vec![];

    for (i, recording) in audio.recordings.iter_mut().enumerate() {
        /*
        let mut frame_count = 0;
        // let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
        let file_frames = recording.iter_mut();
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(file_frame) {
                *sample = *file_sample;
            }
            frame_count += 1;
        }
        */

        for frame in buffer.frames_mut() {
            if audio.index < recording.len() {
                let recorded_frame = recording[audio.index];
                for (sample, recorded_sample) in frame.iter_mut().zip(&recorded_frame) {
                    *sample = *recorded_sample;
                }
                audio.index += 1;
            }
        }

        // This fucks up the performance !!!!!!!!!
        // for i in (0..frame_count).rev() {
        //     recording.remove(i);
        // }
        // =======================================

        // if frame_count < len_frames {
        //     have_ended.push(i);
        // }

        if audio.index == recording.len() {
            have_ended.push(i);
            audio.index = 0;
        }
    }

    // Remove all sounds that have ended
    for i in have_ended.into_iter().rev() {
        audio.recordings.remove(i);
    }
}
