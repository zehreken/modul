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
    consumer_input: Consumer<f32>,
    producer_output: Producer<f32>,
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

        let ring_buffer_input = RingBuffer::new(BUFFER_CAPACITY);
        let (producer_input, consumer_input) = ring_buffer_input.split();

        let ring_buffer_output = RingBuffer::new(BUFFER_CAPACITY);
        let (producer_output, consumer_output) = ring_buffer_output.split();

        let (tape_sender, tape_receiver) = channel();

        let (time_sender, time_receiver) = channel();

        let input_stream = create_input_stream(&input_device, &config, producer_input);
        // let output_stream =
        //     create_output_stream(&output_device, &config, consumer_output, time_sender);
        let output_stream = create_output_stream_2(&output_device, &config, tape_receiver);

        input_stream.pause().unwrap();
        // output_stream.pause().unwrap();

        Modul {
            recording_tape,
            tapes,
            input_stream,
            output_stream,
            consumer_input,
            producer_output,
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
        while !self.consumer_input.is_empty() {
            for frame in self.consumer_input.pop() {
                self.recording_tape.audio.push(frame);
            }
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn record(&mut self) {
        if self.modul_state.is_input_playing {
            self.input_stream.pause().unwrap();
            self.modul_state.is_input_playing = false;
            self.tape_sender.send(self.recording_tape.clone()).unwrap();
        } else {
            println!("recorded frames: {}", self.recording_tape.audio.len());
            self.recording_tape.clear();
            self.input_stream.play().unwrap();
            self.modul_state.is_input_playing = true;
        }
    }

    pub fn play(&self) {}
}
