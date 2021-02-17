pub mod utils {
    use crate::tape::tape::Tape;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{Device, Stream, StreamConfig};
    use ringbuf::{Consumer, Producer, RingBuffer};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::{channel, Receiver, Sender};
    use std::sync::Arc;

    pub const TAPE_LENGTH: usize = 44100 * 2 * 4; // sample_rate * channels * seconds
    /// ATTENTION:
    /// If buffer capacity and update frequency is related, if update frequency is low
    /// then the buffer will not be emptied fast enough and some input will be lost
    pub const BUFFER_CAPACITY: usize = 4096 * 8;

    // think about sending the time also from the input_stream
    // something like this Producer<(usize, f32)>
    // you can use this also for output_stream Consumer<(usize, f32)>
    pub fn create_input_stream(
        input_device: &Device,
        config: &StreamConfig,
        mut producer: Producer<f32>,
    ) -> Stream {
        let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.push(sample).is_err() {
                    output_fell_behind = true;
                }
            }

            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
        };

        input_device
            .build_input_stream(config, input_data, err_fn)
            .unwrap()
    }

    pub fn create_output_stream(
        output_device: &Device,
        config: &StreamConfig,
        mut consumer: Consumer<(usize, f32)>,
        audio_index: Arc<AtomicUsize>,
    ) -> Stream {
        let mut tape = Tape::<f32>::new(0.0, TAPE_LENGTH);
        let mut index = 0;
        let output_data = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                *sample = tape.audio[index];
                // println!("{}", sample);
                index += 1;

                if index == tape.audio.len() {
                    index = 0;
                }
            }

            audio_index.store(index, Ordering::SeqCst);

            while !consumer.is_empty() {
                match consumer.pop() {
                    Some(t) => tape.audio[t.0] = t.1,
                    None => {}
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

    pub fn merge_tapes(tapes: &[Tape<f32>]) -> Tape<f32> {
        let mut sum_tape: Tape<f32> = Tape {
            volume: 1.0,
            audio: vec![0.0; TAPE_LENGTH],
        };

        for tape in tapes {
            for (sum, sample) in sum_tape.audio.iter_mut().zip(tape.audio.iter()) {
                *sum += *sample;
            }
        }

        sum_tape
    }

    pub fn write_tape(tape: &Tape<f32>, name: &str) {
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
}
