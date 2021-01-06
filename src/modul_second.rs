use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use nannou_audio::sample::ring_buffer;
use ringbuf::{Consumer, Producer, RingBuffer};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

const BUFFER_CAPACITY: usize = 4096;
const TAPE_LENGTH: usize = 44100 * 4;

pub struct TapeModel {
    pub tapes: [Tape<f32>; 4],
}

pub struct Modul {
    tapes: TapeModel,
    input_stream: Stream,
    output_stream: Stream,
}

impl Modul {
    pub fn new() -> Self {
        let tapes = TapeModel {
            tapes: [
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
            ],
        };

        let host = cpal::default_host();

        let input_device = host.default_input_device().unwrap();
        let output_device = host.default_output_device().unwrap();

        let config: StreamConfig = input_device.default_input_config().unwrap().into();

        let ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (producer, consumer) = ring_buffer.split();

        let input_stream = create_input_stream(&input_device, &config, producer);
        let output_stream = create_output_stream(&output_device, &config, consumer);

        Modul {
            tapes,
            input_stream,
            output_stream,
        }
    }

    pub fn get_time(&self) -> f32 {
        0.0
    }

    pub fn play_streams(&self) {
        self.input_stream.play().unwrap();
        self.output_stream.play().unwrap();
    }

    pub fn pause_streams(&self) {
        self.input_stream.pause().unwrap();
        self.output_stream.pause().unwrap();
    }
}

fn create_input_stream(
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

fn create_output_stream(
    output_device: &Device,
    config: &StreamConfig,
    mut consumer: Consumer<f32>,
    // time_sender: Sender<f32>,
) -> Stream {
    let output_data = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }

        // time_sender.send(512 as f32 / 44100 as f32).unwrap();

        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };

    output_device
        .build_output_stream(config, output_data, err_fn)
        .unwrap()
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occured on stream: {}", err);
}
