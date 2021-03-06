pub mod utils {
    use crate::tape::tape::Tape;
    use cpal::traits::DeviceTrait;
    use cpal::{Device, Stream, StreamConfig};
    use ringbuf::{Consumer, Producer};

    // pub const TAPE_LENGTH: usize = 44100 * 2 * 8; // sample_rate * channels * seconds
    // Currently it is alwasy 4/4 time signature
    // pub const BPM: f32 = 120.0;
    // pub const BAR_LENGTH_SECONDS: f32 = 4.0 * 60.0 / BPM; // beats * seconds per beat(60.0 / BPM)
    // pub const BAR_COUNT: usize = 4;
    /// ATTENTION:
    /// If buffer capacity and update frequency is related, if update frequency is low
    /// then the buffer will not be emptied fast enough and some input will be lost
    pub const BUFFER_CAPACITY: usize = 4096 * 8;

    pub fn create_input_stream_live(
        input_device: &Device,
        config: &StreamConfig,
        tape_length: usize,
        mut producer: Producer<(usize, f32)>,
    ) -> Stream {
        let mut index = 0;
        let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut consumer_fell_behind = false;
            for &sample in data {
                if producer.push((index, sample)).is_err() {
                    consumer_fell_behind = true;
                }
                index += 1;

                if index == tape_length {
                    index = 0;
                }
            }

            if consumer_fell_behind {
                eprintln!("main audio thread fell behind");
            }
        };

        input_device
            .build_input_stream(config, input_data, err_fn)
            .unwrap()
    }

    pub fn create_output_stream_live(
        output_device: &Device,
        config: &StreamConfig,
        mut consumer: Consumer<f32>,
    ) -> Stream {
        let output_data = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => s,
                    None => 0.0,
                }
            }
        };

        output_device
            .build_output_stream(config, output_data, err_fn)
            .unwrap()
    }

    pub fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occured on stream: {}", err);
    }

    pub fn _merge_tapes(tapes: &[Tape<f32>], tape_length: usize) -> Tape<f32> {
        let mut sum_tape: Tape<f32> = Tape::new(0.0, tape_length);

        for tape in tapes {
            for (sum, sample) in sum_tape.audio.iter_mut().zip(tape.audio.iter()) {
                *sum += *sample;
            }
        }

        sum_tape
    }

    pub fn _write_tape(tape: &Tape<f32>, name: &str) {
        let spec = hound::WavSpec {
            channels: 4,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();
        for frame in tape.audio.iter() {
            let sample = frame;
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }

    pub fn write(recording: &Vec<f32>, name: &str) {
        let spec = hound::WavSpec {
            channels: 4,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();
        for sample in recording.iter() {
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }
}
