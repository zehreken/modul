use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::mpsc::{channel, Receiver, Sender};

const BUFFER_CAPACITY: usize = 4096;
const TAPE_LENGTH: usize = 44100 * 4;

struct ModulState {
    is_input_playing: bool,
    is_output_playing: bool,
}

pub struct TapeModel {
    pub tapes: [Tape<f32>; 4],
}

pub struct Modul {
    recording_tape: Tape<f32>,
    tapes: TapeModel,
    input_stream: Stream,
    output_stream: Stream,
    input_consumer: Consumer<f32>,
    output_producer: Producer<f32>,
    tape_sender: Sender<Tape<f32>>,
    time_receiver: Receiver<f32>,
    time: f32,
    modul_state: ModulState,
}

impl Modul {
    pub fn new() -> Self {
        let recording_tape = Tape::<f32>::new(0.0, TAPE_LENGTH);
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

        let input_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let output_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let (tape_sender, tape_receiver) = channel();

        let (time_sender, time_receiver) = channel();

        let input_stream = create_input_stream(&input_device, &config, input_producer);
        // let output_stream =
        //     create_output_stream(&output_device, &config, consumer_output, time_sender);
        let output_stream =
            create_output_stream_2(&output_device, &config, tape_receiver, time_sender);

        input_stream.pause().unwrap();
        // output_stream.pause().unwrap();

        Modul {
            recording_tape,
            tapes,
            input_stream,
            output_stream,
            input_consumer,
            output_producer,
            tape_sender,
            time_receiver,
            time: 0.0,
            modul_state: ModulState {
                is_input_playing: false,
                is_output_playing: false,
            },
        }
    }

    pub fn update(&mut self) {
        for v in self.time_receiver.try_iter() {
            self.time += v;
        }

        // println!("remaining space: {}", self.consumer_input.remaining());
        while !self.input_consumer.is_empty() {
            for frame in self.input_consumer.pop() {
                self.recording_tape.audio.push(frame);
            }
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn record(&mut self) {
        if self.modul_state.is_input_playing {
            // println!("recorded frames: {}", self.recording_tape.audio.len());
            self.input_stream.pause().unwrap();
            self.modul_state.is_input_playing = false;

            let mut audio = vec![0.0; TAPE_LENGTH];
            for i in 0..TAPE_LENGTH {
                if i < self.recording_tape.audio.len() {
                    audio[i] += self.recording_tape.audio[i];
                }
            }
            let temp_tape = Tape { volume: 1.0, audio };
            self.tape_sender.send(temp_tape).unwrap();
        } else {
            self.recording_tape.clear();
            self.input_stream.play().unwrap();
            self.modul_state.is_input_playing = true;
        }
    }

    pub fn play(&self) {}
}
