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

pub fn start(time_sender: Sender<f32>, key_receiver: Receiver<u8>) {
    println!("Starting modul_cpal");
    let host = cpal::default_host();

    let input_device = host.default_input_device().unwrap();

    let output_device = host.default_output_device().unwrap();

    println!("Using input device: \"{:?}\"", input_device.name());
    println!("Using output device: \"{:?}\"", output_device.name());

    let config: StreamConfig = input_device.default_input_config().unwrap().into();

    const BUFFER_CAPACITY: usize = 4096;
    const TAPE_LENGTH: usize = 44100 * 4; // 4 seconds
    let latency_samples = 100;
    let ring_one = RingBuffer::new(BUFFER_CAPACITY);
    let (mut producer_one, mut consumer_one) = ring_one.split();

    let ring_two = RingBuffer::new(BUFFER_CAPACITY);
    let (mut producer_two, consumer_two) = ring_two.split();

    for _ in 0..latency_samples {
        producer_one.push(0.0).unwrap();
    }

    let input_stream = create_input_stream(&input_device, &config, producer_one);
    let output_stream = create_output_stream(&output_device, &config, consumer_two, time_sender);

    let mut modul = Modul {
        tape_model: TapeModel {
            tapes: [
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
                Tape::<f32>::new(0.0, TAPE_LENGTH),
            ],
        },
    };

    let mut audio_index = 0;
    loop {
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

        while consumer_one.remaining() > 0 {
            for s in consumer_one.pop() {
                modul.tape_model.tapes[0].audio[audio_index] = s;
                audio_index += 1;
                if audio_index == TAPE_LENGTH {
                    audio_index = 0;
                }
            }
        }

        for i in 0..44100 {
            producer_two.push(modul.tape_model.tapes[0].audio[i]).unwrap();
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
    time_sender: Sender<f32>,
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

        time_sender.send(512 as f32 / 44100 as f32).unwrap();

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
