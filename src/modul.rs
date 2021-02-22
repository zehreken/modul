use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::Arc;
use std::sync::{
    atomic::AtomicBool,
    mpsc::{channel, Receiver, Sender},
};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

pub struct TapeModel {
    pub tapes: [Tape<f32>; 4],
}

/// Used to transfer data to the audio thread
/// In this context audio thread is the thread that
/// communicates with input and output streams(each has its own thread)
struct AudioModel {
    recording_tape: Vec<(usize, f32)>,
    tape_model: TapeModel,
    input_consumer: Consumer<(usize, f32)>,
    key_receiver: Receiver<ModulAction>,
    is_recording: Arc<AtomicBool>,
    selected_tape: usize,
    output_producer: Producer<f32>,
    audio_index: Arc<AtomicUsize>,
    writing_tape: Vec<f32>,
}

enum ModulAction {
    SelectTape(usize),
    Record,
    _Play,
    Write,
    _ClearAll,
    Clear,
    Mute,
    Unmute,
}

impl AudioModel {
    pub fn update(&mut self) {
        while !self.input_consumer.is_empty() {
            let mut audio_index = 0;
            for t in self.input_consumer.pop() {
                if self.is_recording.load(Ordering::SeqCst) {
                    self.recording_tape.push(t);
                }
                // send audio to output
                let mut s: f32 = 0.0;
                for tape in self.tape_model.tapes.iter() {
                    s += tape.audio[t.0] * tape.volume;
                }
                let r = self.output_producer.push(t.1 + s);
                match r {
                    Ok(_) => {}
                    Err(_e) => eprintln!("error: {}", self.output_producer.len()),
                }
                self.writing_tape.push(t.1 + s);

                audio_index = t.0;
            }
            self.audio_index.store(audio_index, Ordering::SeqCst);
        }

        self.check_input();
    }

    fn check_input(&mut self) {
        for c in self.key_receiver.try_iter() {
            match c {
                ModulAction::Record => {
                    if self.is_recording.load(Ordering::SeqCst) {
                        println!("stop recording {}", self.selected_tape);
                        self.is_recording.store(false, Ordering::SeqCst);

                        let mut audio = vec![0.0; TAPE_LENGTH];
                        // This forces fake stereo
                        // for i in (0..self.recording_tape.len()).step_by(2) {
                        //     let mut index = self.recording_tape[i].0;
                        //     let sample = self.recording_tape[i].1 + self.recording_tape[i + 1].1;
                        //     for j in 0..2 {
                        //         index = index + j;
                        //         index %= TAPE_LENGTH;
                        //         audio[index] = sample;
                        //     }
                        // }
                        // Without forcing fake stereo
                        for t in self.recording_tape.iter() {
                            audio[t.0] = t.1;
                        }

                        self.tape_model.tapes[self.selected_tape].audio = audio;
                        self.recording_tape.clear();
                    } else {
                        println!("start recording {}", self.selected_tape);
                        self.is_recording.store(true, Ordering::SeqCst);
                        self.recording_tape.clear();
                    }
                }
                ModulAction::Write => {
                    // let tape = merge_tapes(&self.tape_model.tapes);
                    // write_tape(&tape, "test");
                    write(&self.writing_tape, "full");
                }
                ModulAction::_ClearAll => {
                    println!("clear all");
                    for tape in self.tape_model.tapes.iter_mut() {
                        tape.clear(0.0);
                    }
                }
                ModulAction::Clear => {
                    println!("clear {}", self.selected_tape);
                    self.tape_model.tapes[self.selected_tape].clear(0.0);
                }
                ModulAction::Mute => {
                    println!("mute {}", self.selected_tape);
                    self.tape_model.tapes[self.selected_tape].mute();
                }
                ModulAction::Unmute => {
                    println!("unmute {}", self.selected_tape);
                    self.tape_model.tapes[self.selected_tape].unmute();
                }
                ModulAction::SelectTape(tape) => {
                    self.selected_tape = tape;
                }
                _ => {}
            }
        }
    }
}

pub struct Modul {
    _input_stream: Stream,
    _output_stream: Stream,
    time: f32,
    audio_index: Arc<AtomicUsize>,
    key_sender: Sender<ModulAction>,
    is_recording: Arc<AtomicBool>,
}

impl Modul {
    pub fn new() -> Self {
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
        println!("input channel count: {}", config.channels);
        println!("sample rate: {:?}", config.sample_rate);

        let input_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let (key_sender, key_receiver) = channel();

        let output_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let input_stream = create_input_stream_live(&input_device, &config, input_producer);

        let audio_index = Arc::new(AtomicUsize::new(0));

        let output_stream = create_output_stream_live(&output_device, &config, output_consumer);

        let is_recording = Arc::new(AtomicBool::new(false));

        let mut audio_model: AudioModel = AudioModel {
            recording_tape,
            tape_model,
            input_consumer,
            key_receiver,
            is_recording: Arc::clone(&is_recording),
            selected_tape: 0,
            output_producer,
            audio_index: Arc::clone(&audio_index),
            writing_tape: vec![],
        };

        std::thread::spawn(move || loop {
            audio_model.update();
            std::thread::sleep(Duration::from_millis(1));
        });

        Modul {
            _input_stream: input_stream,
            _output_stream: output_stream,
            time: 0.0,
            audio_index: Arc::clone(&audio_index),
            is_recording: Arc::clone(&is_recording),
            key_sender,
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time
    }

    pub fn get_audio_index(&self) -> usize {
        self.audio_index.load(Ordering::SeqCst)
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    pub fn set_selected_tape(&mut self, selected_tape: usize) {
        self.key_sender
            .send(ModulAction::SelectTape(selected_tape))
            .unwrap();
    }

    pub fn record(&mut self) {
        self.key_sender.send(ModulAction::Record).unwrap();
    }

    pub fn play(&self) {}

    pub fn write(&self) {
        self.key_sender.send(ModulAction::Write).unwrap();
    }

    pub fn _clear_all(&self) {
        self.key_sender.send(ModulAction::_ClearAll).unwrap();
    }

    pub fn clear(&self) {
        self.key_sender.send(ModulAction::Clear).unwrap();
    }

    pub fn mute(&self) {
        self.key_sender.send(ModulAction::Mute).unwrap();
    }

    pub fn unmute(&self) {
        self.key_sender.send(ModulAction::Unmute).unwrap();
    }
}
