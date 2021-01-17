use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::mpsc::{channel, Receiver, Sender};

struct ModulState {
    is_input_playing: bool,
    is_output_playing: bool,
}

pub struct TapeModel {
    pub tapes: [Tape<f32>; 4],
}

pub struct OutputModel {
    pub output_producer: Producer<(usize, f32)>,
    pub temp_tape: Tape<f32>,
    pub audio_index: usize,
}

pub struct Modul {
    recording_tape: Tape<f32>,
    tape_model: TapeModel,
    input_stream: Stream,
    output_stream: Stream,
    input_consumer: Consumer<(usize, f32)>,
    // output_producer: Producer<(usize, f32)>,
    selected_tape: usize,
    tape_sender: Sender<Tape<f32>>,
    time_receiver: Receiver<f32>,
    index_receiver: Receiver<usize>,
    time: f32,
    audio_index: usize,
    modul_state: ModulState,
    output_model: OutputModel,
}

impl Modul {
    pub fn new() -> Self {
        let recording_tape = Tape::<f32>::new(0.0, TAPE_LENGTH);
        let tape_model = TapeModel {
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
        let (index_sender, index_receiver) = channel();

        let input_stream = create_input_stream(&input_device, &config, input_producer);
        // let output_stream =
        //     create_output_stream(&output_device, &config, consumer_output, time_sender);

        // let output_stream =
        //     create_output_stream_2(&output_device, &config, tape_receiver, time_sender);

        let output_stream =
            create_output_stream_3(&output_device, &config, output_consumer, index_sender);

        input_stream.pause().unwrap();
        // output_stream.pause().unwrap();

        Modul {
            recording_tape,
            tape_model,
            input_stream,
            output_stream,
            input_consumer,
            // output_producer,
            selected_tape: 0,
            tape_sender,
            time_receiver,
            index_receiver,
            time: 0.0,
            audio_index: 0,
            modul_state: ModulState {
                is_input_playing: false,
                is_output_playing: false,
            },
            output_model: OutputModel {
                output_producer,
                temp_tape: Tape::<f32>::new(0.0, TAPE_LENGTH),
                audio_index: 0,
            },
        }
    }

    pub fn update(&mut self) {
        // for v in self.time_receiver.try_iter() {
        //     self.time += v;
        // }

        for v in self.index_receiver.try_iter() {
            self.audio_index += v;
            if self.audio_index > TAPE_LENGTH {
                self.audio_index = 0;
            }
        }

        // println!("remaining space: {}", self.consumer_input.remaining());
        while !self.input_consumer.is_empty() {
            for element in self.input_consumer.pop() {
                self.recording_tape.audio[element.0] = element.1;
            }
        }

        if self.output_model.audio_index < TAPE_LENGTH {
            for _ in 0..4096 {
                if self.output_model.audio_index < TAPE_LENGTH {
                    let r = self.output_model.output_producer.push((
                        self.output_model.audio_index,
                        self.output_model.temp_tape.audio[self.output_model.audio_index],
                    ));
                    match r {
                        Ok(_) => self.output_model.audio_index += 1,
                        Err(_) => {}
                    }
                }
            }
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_audio_index(&self) -> usize {
        self.audio_index
    }

    pub fn set_selected_tape(&mut self, selected_tape: usize) {
        self.selected_tape = selected_tape;
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
            self.tape_model.tapes[self.selected_tape].audio = audio;
            // let temp_tape = merge_tapes(&self.tape_model.tapes);
            // self.tape_sender.send(temp_tape).unwrap();
            self.output_model.temp_tape = merge_tapes(&self.tape_model.tapes);
            self.output_model.audio_index = 0;
        } else {
            // self.recording_tape.clear();
            self.input_stream.play().unwrap();
            self.modul_state.is_input_playing = true;
        }
    }

    pub fn play(&self) {}

    pub fn write(&self) {
        let tape = merge_tapes(&self.tape_model.tapes);
        write_tape(&tape, "test");
    }
}
