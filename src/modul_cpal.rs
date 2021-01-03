use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

use cpal::Stream;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};
use ringbuf::{Consumer, Producer, RingBuffer};

use crate::tape::tape::Tape;

struct TapeModel {
    pub tapes: [Tape<f32>; 4],
}
struct Modul {
    tape_model: TapeModel,
}

pub fn start(sender: Sender<f32>, key_receiver: Receiver<u8>) {
    println!("Starting modul_cpal");
    let host = cpal::default_host();

    let input_device = host.default_input_device().unwrap();

    let output_device = host.default_output_device().unwrap();

    println!("Using input device: \"{:?}\"", input_device.name());
    println!("Using output device: \"{:?}\"", output_device.name());

    let config: StreamConfig = input_device.default_input_config().unwrap().into();

    const BUFFER_CAPACITY: usize = 4096;
    let latency_samples = 100;
    let ring = RingBuffer::new(BUFFER_CAPACITY);
    let (mut producer, consumer) = ring.split();

    for _ in 0..latency_samples {
        producer.push(0.0).unwrap();
    }

    let input_stream = create_input_stream(&input_device, &config, producer);
    let output_stream = create_output_stream(&output_device, &config, consumer);

    let modul = Modul {
        tape_model: TapeModel {
            tapes: [
                Tape::<f32>::new(0.0, 4096),
                Tape::<f32>::new(0.0, 4096),
                Tape::<f32>::new(0.0, 4096),
                Tape::<f32>::new(0.0, 4096),
            ],
        },
    };

    let instant = std::time::Instant::now();

    loop {
        let r = sender.send(instant.elapsed().as_secs_f32());
        if r.is_err() {
            dbg!(r.err());
        }

        let r = key_receiver.try_recv();
        match r {
            Ok(v) => {
                println!("{}", v);
                if v == 1 {
                    input_stream.play().unwrap();
                    output_stream.play().unwrap();
                } else {
                    input_stream.pause().unwrap();
                    output_stream.pause().unwrap();
                }
            }
            Err(_) => {}
        }

        thread::sleep(std::time::Duration::from_millis(33));
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
