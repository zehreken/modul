use super::audio_model::*;
use super::modul_utils::utils::*;
use super::tape::tape::Tape;
use super::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::RingBuffer;
use std::sync::{
    atomic::AtomicBool,
    mpsc::{channel, Sender},
};
use std::sync::{Arc, Mutex};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

pub struct Modul {
    pub tape_length: usize,
    input_stream: Stream,
    output_stream: Stream,
    _time: f32,
    audio_index: Arc<AtomicUsize>,
    key_sender: Sender<ModulAction>,
    is_recording: Arc<AtomicBool>,
    is_recording_playback: Arc<AtomicBool>,
    sample_averages: Arc<Mutex<[f32; 4]>>,
}

impl Modul {
    pub fn new(config: Config) -> Self {
        let host = cpal::default_host();

        let input_device = host.default_input_device().unwrap();
        let output_device = host.default_output_device().unwrap();

        let input_config: StreamConfig = input_device.default_input_config().unwrap().into();
        println!("input channel count: {}", input_config.channels);
        println!("input sample rate: {:?}", input_config.sample_rate);
        let bar_length_seconds = 4.0 * 60.0 / config.bpm; // beats * seconds per beat(60.0 / BPM)
                                                          // sample rate * channel count(4 on personal mac) * bar length in seconds * bar count
        let tape_length: usize = (input_config.sample_rate.0 as f32
            * input_config.channels as f32
            * bar_length_seconds
            * config.bar_count as f32) as usize;
        println!("by 4: {}", tape_length % 4);

        let recording_tape = vec![];
        let tape_model = TapeModel {
            tapes: [
                Tape::<f32>::new(0.0, tape_length),
                Tape::<f32>::new(0.0, tape_length),
                Tape::<f32>::new(0.0, tape_length),
                Tape::<f32>::new(0.0, tape_length),
            ],
        };

        println!(
            "tape length: {}, bar length: {} seconds",
            tape_length, bar_length_seconds
        );

        let input_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let (key_sender, key_receiver) = channel();

        let output_ring_buffer = RingBuffer::new(BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let input_stream =
            create_input_stream_live(&input_device, &input_config, tape_length, input_producer);

        let audio_index = Arc::new(AtomicUsize::new(0));

        let output_stream =
            create_output_stream_live(&output_device, &input_config, output_consumer);

        let is_recording = Arc::new(AtomicBool::new(false));
        let is_recording_playback = Arc::new(AtomicBool::new(false));
        let sample_averages = Arc::new(Mutex::new([0.0; 4]));

        let mut audio_model: AudioModel = AudioModel {
            tape_length,
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
            tape_length,
            input_stream,
            output_stream,
            _time: 0.0,
            audio_index: Arc::clone(&audio_index),
            is_recording: Arc::clone(&is_recording),
            is_recording_playback: Arc::clone(&is_recording_playback),
            key_sender,
            sample_averages: Arc::clone(&sample_averages),
        }
    }

    pub fn _get_time(&self) -> f32 {
        self._time
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
        *self.sample_averages.lock().unwrap()
    }
}
