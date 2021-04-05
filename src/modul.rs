use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::sync::{
    atomic::AtomicBool,
    mpsc::{channel, Receiver, Sender},
};
use std::sync::{Arc, Mutex};
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
    is_recording_playback: Arc<AtomicBool>,
    selected_tape: usize,
    output_producer: Producer<f32>,
    audio_index: Arc<AtomicUsize>,
    writing_tape: Vec<f32>,
    sample_averages: Arc<Mutex<[f32; 4]>>,
}

enum ModulAction {
    SelectTape(usize),
    Record,
    _Pause,
    _Play,
    Playback,
    Write,
    _ClearAll,
    Clear,
    Mute,
    Unmute,
    VolumeUp,
    VolumeDown,
}

impl AudioModel {
    pub fn update(&mut self) {
        let mut sample_averages = [0.0; 4];
        let sample_count = self.input_consumer.len();
        while !self.input_consumer.is_empty() {
            let mut audio_index = 0;
            for t in self.input_consumer.pop() {
                if self.is_recording.load(Ordering::SeqCst) {
                    self.recording_tape.push(t);
                }
                // send audio to output
                let mut sample: f32 = 0.0;
                for (tape, average) in self.tape_model.tapes.iter().zip(sample_averages.iter_mut())
                {
                    *average += tape.audio[t.0] * tape.get_volume();
                    sample += tape.audio[t.0] * tape.get_volume();
                }

                let r = self.output_producer.push(sample + t.1);
                match r {
                    Ok(_) => {}
                    Err(_e) => eprintln!("error: {}", self.output_producer.len()),
                }
                if self.is_recording_playback.load(Ordering::SeqCst) {
                    sample += t.1;
                }
                self.writing_tape.push(sample);

                audio_index = t.0;
            }
            self.audio_index.store(audio_index, Ordering::SeqCst);
        }

        if sample_count > 0 {
            for average in sample_averages.iter_mut() {
                *average /= sample_count as f32;
                *average += 0.02;
                // println!("{}", average);
            }
            *self.sample_averages.lock().unwrap() = sample_averages;
        }

        self.check_input();
    }

    fn check_input(&mut self) {
        for c in self.key_receiver.try_iter() {
            match c {
                ModulAction::Record => {
                    if self.is_recording.load(Ordering::SeqCst) {
                        self.is_recording.store(false, Ordering::SeqCst);

                        let mut audio = vec![0.0; TAPE_LENGTH];

                        for t in self.recording_tape.iter() {
                            audio[t.0] = t.1;
                        }

                        self.tape_model.tapes[self.selected_tape].audio = audio;
                        self.recording_tape.clear();
                    } else {
                        self.is_recording.store(true, Ordering::SeqCst);
                        self.recording_tape.clear();
                    }
                }
                ModulAction::Playback => {
                    self.is_recording_playback.store(
                        !self.is_recording_playback.load(Ordering::SeqCst),
                        Ordering::SeqCst,
                    );
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
                ModulAction::VolumeUp => {
                    self.tape_model.tapes[self.selected_tape].volume_up();
                }
                ModulAction::VolumeDown => {
                    self.tape_model.tapes[self.selected_tape].volume_down();
                }
                _ => {}
            }
        }
    }
}

pub struct Modul {
    input_stream: Stream,
    output_stream: Stream,
    time: f32,
    audio_index: Arc<AtomicUsize>,
    key_sender: Sender<ModulAction>,
    is_recording: Arc<AtomicBool>,
    is_recording_playback: Arc<AtomicBool>,
    sample_averages: Arc<Mutex<[f32; 4]>>,
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

        println!(
            "tape length: {}, bar length: {} seconds",
            TAPE_LENGTH, BAR_LENGTH_SECONDS
        );

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
        let is_recording_playback = Arc::new(AtomicBool::new(false));
        let sample_averages = Arc::new(Mutex::new([0.0; 4]));

        let mut audio_model: AudioModel = AudioModel {
            recording_tape,
            tape_model,
            input_consumer,
            key_receiver,
            is_recording: Arc::clone(&is_recording),
            is_recording_playback: Arc::clone(&is_recording_playback),
            selected_tape: 0,
            output_producer,
            audio_index: Arc::clone(&audio_index),
            writing_tape: vec![],
            sample_averages: Arc::clone(&sample_averages),
        };

        std::thread::spawn(move || loop {
            audio_model.update();
            std::thread::sleep(Duration::from_millis(1));
        });

        Modul {
            input_stream,
            output_stream,
            time: 0.0,
            audio_index: Arc::clone(&audio_index),
            is_recording: Arc::clone(&is_recording),
            is_recording_playback: Arc::clone(&is_recording_playback),
            key_sender,
            sample_averages: Arc::clone(&sample_averages),
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

    pub fn is_recording_playback(&self) -> bool {
        self.is_recording_playback.load(Ordering::SeqCst)
    }

    pub fn set_selected_tape(&mut self, selected_tape: usize) {
        self.key_sender
            .send(ModulAction::SelectTape(selected_tape))
            .unwrap();
    }

    pub fn record(&mut self) {
        self.key_sender.send(ModulAction::Record).unwrap();
    }

    pub fn pause(&self) {
        // self.key_sender.send(ModulAction::Pause).unwrap();
        self.input_stream.pause().unwrap();
        self.output_stream.pause().unwrap();
    }

    pub fn play(&self) {
        // self.key_sender.send(ModulAction::Play).unwrap();
        self.input_stream.play().unwrap();
        self.output_stream.play().unwrap();
    }

    pub fn record_playback(&self) {
        self.key_sender.send(ModulAction::Playback).unwrap();
    }

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

    pub fn volume_up(&self) {
        self.key_sender.send(ModulAction::VolumeUp).unwrap();
    }

    pub fn volume_down(&self) {
        self.key_sender.send(ModulAction::VolumeDown).unwrap();
    }

    pub fn get_sample_averages(&self) -> [f32; 4] {
        self.sample_averages.lock().unwrap().clone()
    }
}
