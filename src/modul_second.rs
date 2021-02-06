use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::mpsc::{channel, Receiver, Sender};

struct ModulState {
    is_recording: bool,
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

/// Used to transfer necessary data to the audio thread
/// In this context audio thread is the thread that
/// communicates with input and output streams(they are also threads)
struct AudioModel {
    recording_tape: Vec<f32>,
    tape_model: TapeModel,
    input_consumer: Consumer<f32>,
    index_receiver: Receiver<usize>,
    audio_index: usize,
    output_model: OutputModel,
    key_receiver: Receiver<char>,
    modul_state: ModulState,
    selected_tape: usize,
    start_index: usize,
}

impl AudioModel {
    pub fn update(&mut self) {
        for v in self.index_receiver.try_iter() {
            self.audio_index = v;
        }
        // println!("audio_index: {}", self.audio_index);

        while !self.input_consumer.is_empty() {
            for sample in self.input_consumer.pop() {
                if self.modul_state.is_recording {
                    self.recording_tape.push(sample);
                }
            }
        }

        if self.audio_index < TAPE_LENGTH {
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

        for c in self.key_receiver.try_iter() {
            match c {
                'R' => {
                    if self.modul_state.is_recording {
                        println!("stop recording");
                        self.modul_state.is_recording = false;

                        let mut audio = vec![0.0; TAPE_LENGTH];
                        for i in 0..self.recording_tape.len() {
                            let mut index = self.start_index + i;
                            index %= TAPE_LENGTH;
                            audio[index] = self.recording_tape[i];
                        }

                        self.tape_model.tapes[self.selected_tape].audio = audio;
                        self.recording_tape.clear();
                        self.output_model.temp_tape = merge_tapes(&self.tape_model.tapes);
                        self.output_model.audio_index = 0;
                    } else {
                        self.modul_state.is_recording = true;
                        self.recording_tape.clear();
                        self.start_index = self.audio_index % TAPE_LENGTH;
                        println!(
                            "start recording at {0:.2}",
                            self.start_index as f32 / TAPE_LENGTH as f32
                        );
                    }
                }
                '0' => self.selected_tape = 0,
                '1' => self.selected_tape = 1,
                '2' => self.selected_tape = 2,
                '3' => self.selected_tape = 3,
                _ => {}
            }
        }
    }

    fn record(&mut self) {}
}

pub struct Modul {
    // recording_tape: Vec<f32>, // gone
    // tape_model: TapeModel,    // gone
    _input_stream: Stream,
    _output_stream: Stream,
    // input_consumer: Consumer<f32>, // gone
    // selected_tape: usize,
    // index_receiver: Receiver<usize>, // gone
    time: f32,
    audio_index: usize, // obsolete
    // start_index: usize,
    key_sender: Sender<char>,
    // output_model: OutputModel, // gone
}

impl Modul {
    pub fn new() -> Self {
        // let recording_tape = Tape::<f32>::new(0.0, TAPE_LENGTH);
        let recording_tape = vec![];
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
        println!("sample rate: {:?}", config.sample_rate);

        let input_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let output_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let (index_sender, index_receiver) = channel();
        let (key_sender, key_receiver) = channel();

        let input_stream = create_input_stream(&input_device, &config, input_producer);

        let output_stream =
            create_output_stream(&output_device, &config, output_consumer, index_sender);

        // input_stream.pause().unwrap();
        // output_stream.pause().unwrap();

        let output_model = OutputModel {
            output_producer,
            temp_tape: Tape::<f32>::new(0.0, TAPE_LENGTH),
            audio_index: 0,
        };

        let mut audio_model: AudioModel = AudioModel {
            recording_tape,
            tape_model,
            input_consumer,
            index_receiver,
            audio_index: 0,
            output_model,
            key_receiver,
            modul_state: ModulState {
                is_recording: false,
                is_output_playing: false,
            },
            selected_tape: 0,
            start_index: 0,
        };

        std::thread::spawn(move || loop {
            audio_model.update();
        });

        Modul {
            // recording_tape,
            // tape_model,
            _input_stream: input_stream,
            _output_stream: output_stream,
            // input_consumer,
            // selected_tape: 0,
            // index_receiver,
            time: 0.0,
            audio_index: 0,
            // start_index: 0,
            key_sender,
            // output_model: OutputModel {
            //     output_producer,
            //     temp_tape: Tape::<f32>::new(0.0, TAPE_LENGTH),
            //     audio_index: 0,
            // },
        }
    }

    pub fn update(&mut self) {
        /*
        for v in self.index_receiver.try_iter() {
            self.audio_index = v;
            // if self.audio_index >= TAPE_LENGTH {
            //     self.audio_index = 0;
            // }
        }

        while !self.input_consumer.is_empty() {
            for sample in self.input_consumer.pop() {
                self.recording_tape.push(sample);
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
        */
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_audio_index(&self) -> usize {
        self.audio_index
    }

    pub fn set_selected_tape(&mut self, selected_tape: usize) {
        // self.selected_tape = selected_tape;
        self.key_sender
            .send(std::char::from_digit(selected_tape as u32, 10).unwrap())
            .unwrap();
    }

    pub fn record(&mut self) {
        self.key_sender.send('R').unwrap();
        /*
        if self.modul_state.is_input_playing {
            println!("stop recording");
            self.input_stream.pause().unwrap();
            self.modul_state.is_input_playing = false;

            let mut audio = vec![0.0; TAPE_LENGTH];
            for i in 0..self.recording_tape.len() {
                let mut index = self.start_index + i;
                index %= TAPE_LENGTH;
                audio[index] = self.recording_tape[i];
            }
            self.tape_model.tapes[self.selected_tape].audio = audio;
            self.recording_tape.clear();
            self.output_model.temp_tape = merge_tapes(&self.tape_model.tapes);
            self.output_model.audio_index = 0;
        } else {
            self.modul_state.is_input_playing = true;
            self.start_index = self.audio_index % TAPE_LENGTH;
            println!(
                "start recording at {0:.2}",
                self.start_index as f32 / TAPE_LENGTH as f32
            );
            self.input_stream.play().unwrap();
        }
        */
    }

    pub fn play(&self) {}

    pub fn write(&self) {
        /*
        let tape = merge_tapes(&self.tape_model.tapes);
        write_tape(&tape, "test");
        */
    }
}
