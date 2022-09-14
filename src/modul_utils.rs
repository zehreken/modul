pub mod utils {
    use crate::audio_model::Input;
    use crate::tape::Tape;
    use cpal::traits::DeviceTrait;
    use cpal::{Device, Stream, StreamConfig};
    use ringbuf::{Consumer, Producer};

    pub const TAPE_COUNT: usize = 8;
    pub const SAMPLE_GRAPH_SIZE: usize = 200;
    pub const A_FREQ: f32 = 440.0;
    pub const C_FREQ: f32 = 523.25;

    #[derive(Debug)]
    pub enum ModulMessage {
        AudioIndex(usize),
        Recording(bool),
        RecordingPlayback(bool),
        PlayThrough(bool),
        ShowBeat(bool),
        BeatIndex(u32),
        SampleAverages([f32; TAPE_COUNT + 1]),
        SamplesForGraphs([[f32; SAMPLE_GRAPH_SIZE]; TAPE_COUNT]),
    }

    #[derive(Debug)]
    pub enum ModulAction {
        SelectPrimaryTape(usize),
        SelectSecondaryTape(usize),
        MergeTapes,
        Record,
        Playback,
        PlayThrough,
        Write,
        Clear,
        ClearAll,
        Mute,
        Unmute,
        Solo,
        Unsolo,
        VolumeUp,
        VolumeDown,
        StartMetronome,
        StopMetronome,
    }

    pub fn create_input_stream_live(
        input_device: &Device,
        config: &StreamConfig,
        tape_length: usize,
        mut producer: Producer<Input>,
    ) -> Stream {
        let mut index = 0;
        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut consumer_fell_behind = false;
            for &sample in data {
                if producer.push(Input { index, sample }).is_err() {
                    consumer_fell_behind = true;
                }

                index += 1;
                if index == tape_length {
                    index = 0;
                }
            }

            if consumer_fell_behind {
                eprintln!("[AudioIn] Audio processing thread fell behind");
            }
        };

        input_device
            .build_input_stream(config, input_data_fn, err_fn)
            .unwrap()
    }

    pub fn create_output_stream_live(
        output_device: &Device,
        config: &StreamConfig,
        mut consumer: Consumer<f32>,
    ) -> Stream {
        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                *sample = consumer.pop().unwrap_or(0.0);
            }
            // Consumer capacity is equal to 8192, I don't know what I intented to achieve here
            // But this got rid of the glitchy sound
            if consumer.len() > 4096 {
                consumer.discard(4096);
                println!("Discarded 4096 samples, consumer: {}", consumer.len());
            }
        };

        output_device
            .build_output_stream(config, output_data_fn, err_fn)
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
            channels: 4,         // TODO: Fix this hardcoded value
            sample_rate: 44100,  // TODO: Fix this hardcoded value
            bits_per_sample: 16, // TODO: Fix this hardcoded value
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();
        for frame in tape.audio.iter() {
            let sample = frame;
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }

    pub fn write(buffer: &[f32], name: &str) {
        let spec = hound::WavSpec {
            channels: 4,         // TODO: Fix this hardcoded value
            sample_rate: 44100,  // TODO: Fix this hardcoded value
            bits_per_sample: 16, // TODO: Fix this hardcoded value
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();
        for sample in buffer.iter() {
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }
}
