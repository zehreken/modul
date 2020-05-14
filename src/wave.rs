// Cache the sine values for better performance
pub fn audio_sine(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volumne = 0.5;
    let mut frames = Vec::with_capacity(buffer.len());
    for frame in buffer.frames_mut() {
        let amp = (2.0 * PI * audio.phase).sin() as f32;
        audio.phase += audio.hz / sample_rate;
        audio.phase += sample_rate;
        frames.push(amp);
        for channel in frame {
            *channel = amp * volume;
        }
    }

    // audio.sender.send(frames).unwrap();
}