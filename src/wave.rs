use super::envelope::*;
use nannou_audio::Buffer;
use std::f64::consts::PI;

pub struct Audio {
    pub phase: f64,
    pub hz: f64,
}

pub struct WaveModel {
    pub envelopes: Vec<Envelope>,
}

pub fn audioE(audio: &mut WaveModel, buffer: &mut Buffer) {
    let mut finished_env = Vec::new();
    for (i, env) in &mut audio.envelopes.iter_mut().enumerate() {
        let sample_rate = buffer.sample_rate() as f64;
        let volume = 0.5;
        for frame in buffer.frames_mut() {
            let amp = (2.0 * PI * env.phase).sin() as f32;
            env.phase += env.hz / sample_rate;
            env.phase %= sample_rate;
            let passed = (std::time::Instant::now() - env.start).as_secs_f32();
            for channel in frame {
                *channel = amp * volume;
            }

            if passed >= env.duration {
                finished_env.push(i);
                // To prevent pushing the same index twice
                break;
            }
        }
    }

    for i in finished_env.into_iter().rev() {
        audio.envelopes.remove(i);
    }
}

// Cache the sine values for better performance
pub fn audio_sine(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len()); // This is used for visualization
    for frame in buffer.frames_mut() {
        let amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

pub fn audio_square(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let mut amp = 0f32;
        for s in (1..50).step_by(2) {
            amp += (2.0 * PI * audio.phase * s as f64).sin() as f32 / s as f32;
        }
        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}

fn audio_triangle(audio: &mut Audio, buffer: &mut Buffer) {}
fn audio_saw_tooth(audio: &mut Audio, buffer: &mut Buffer) {}
