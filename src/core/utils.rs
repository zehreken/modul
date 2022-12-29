use crate::core::audio_model::Input;
use crate::features::Tape;
use cpal::traits::DeviceTrait;
use cpal::{Device, Stream, StreamConfig};
use ringbuf::{HeapConsumer, HeapProducer};
use std::path::Path;

pub const TAPE_COUNT: usize = 8;
pub const SAMPLE_GRAPH_SIZE: usize = 100;
pub const A_FREQ: f32 = 440.0;
pub const C_FREQ: f32 = 523.25;
pub const CHANNELS: u16 = 4;
pub const SAMPLE_RATE: u32 = 44100;
pub const BITS_PER_SAMPLE: u16 = 16;

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
    ToggleMute,
    ToggleSolo,
    VolumeUp,
    VolumeDown,
    StartMetronome,
    StopMetronome,
}

pub fn create_input_stream_live(
    input_device: &Device,
    config: &StreamConfig,
    tape_length: usize,
    mut producer: HeapProducer<Input>,
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
            println!("[AudioIn] Audio processing thread fell behind");
        }
    };

    input_device
        .build_input_stream(config, input_data_fn, err_fn)
        .unwrap()
}

pub fn create_output_stream_live(
    output_device: &Device,
    config: &StreamConfig,
    mut consumer: HeapConsumer<f32>,
) -> Stream {
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in data {
            *sample = consumer.pop().unwrap_or(0.0);
        }
        // Consumer capacity is equal to 8192, I don't know what I intented to achieve here
        // But this got rid of the glitchy sound
        if consumer.len() > 4096 {
            consumer.skip(4096);
            println!("Skipped 4096 samples, consumer: {}", consumer.len());
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

pub fn load_image(path: &Path) -> image::DynamicImage {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    image::open(path).expect("image not found")
}

pub fn load_image_for_ui(path: &Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn _write_tape(tape: &Tape<f32>, name: &str) {
    let spec = hound::WavSpec {
        channels: CHANNELS,               // TODO: Fix this hardcoded value
        sample_rate: SAMPLE_RATE,         // TODO: Fix this hardcoded value
        bits_per_sample: BITS_PER_SAMPLE, // TODO: Fix this hardcoded value
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("out/{}.wav", name), spec).unwrap();
    for frame in tape.audio.iter() {
        let sample = frame;
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

pub fn write(buffer: &[f32], name: &str) {
    let spec = hound::WavSpec {
        channels: CHANNELS,               // TODO: Fix this hardcoded value
        sample_rate: SAMPLE_RATE,         // TODO: Fix this hardcoded value
        bits_per_sample: BITS_PER_SAMPLE, // TODO: Fix this hardcoded value
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("out/{}.wav", name), spec).unwrap();
    for sample in buffer.iter() {
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
