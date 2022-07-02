use super::audio_model::*;
use super::modul_utils::utils::*;
use super::Config;
use crate::metronome::Metronome;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Stream, StreamConfig};
use ringbuf::RingBuffer;
use std::sync::atomic::AtomicU32;
use std::sync::{
    atomic::AtomicBool,
    mpsc::{channel, Sender},
};
use std::sync::{Arc, Mutex};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

pub struct Stats {
    pub bpm: u16,
    pub bar_count: usize,
    pub bar_length: f32,
}

pub struct Modul {
    pub tape_length: usize,
    input_stream: Stream,
    output_stream: Stream,
    _time: f32,
    audio_index: Arc<AtomicUsize>,
    key_sender: Sender<ModulAction>,
    is_recording: Arc<AtomicBool>,
    is_recording_playback: Arc<AtomicBool>,
    is_play_through: Arc<AtomicBool>,
    sample_averages: Arc<Mutex<[f32; TAPE_COUNT + 1]>>,
    _show_beat: Arc<AtomicBool>,
    beat_index: Arc<AtomicU32>,
    pub stats: Stats,
}

impl Modul {
    pub fn new(config: Config) -> Self {
        let host = cpal::default_host();

        let input_device = host.default_input_device().unwrap();
        let output_device = host.default_output_device().unwrap();

        let mut input_config: StreamConfig = input_device.default_input_config().unwrap().into();
        println!("input channel count: {}", input_config.channels);
        println!("input sample rate: {:?}", input_config.sample_rate);
        let bar_length = 4.0 * 60.0 / config.bpm as f32; // bar length in seconds, beats * seconds per beat(60.0 / BPM)

        let output_config: StreamConfig = input_device.default_output_config().unwrap().into();

        /*
        ATTENTION:
        If buffer capacity and update frequency is related, if update frequency is low
        then the buffer will not be emptied fast enough and some input will be lost
        This is unnecesary since I don't push to the buffer if buffer.len() is 2048
        */
        const BUFFER_SIZE: u32 = 128; // Suggested buffer size for recording is 128, in my tests even 32 works fine
        const RING_BUFFER_CAPACITY: usize = 8192;

        input_config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);

        let stats = Stats {
            bpm: config.bpm,
            bar_count: config.bar_count,
            bar_length,
        };

        // sample rate * channel count(4 on personal mac) * bar length in seconds * bar count
        let mut tape_length: usize = (input_config.sample_rate.0 as f32
            * input_config.channels as f32
            * bar_length
            * config.bar_count as f32) as usize;
        tape_length -= tape_length % input_config.channels as usize;

        let recording_tape = vec![];
        let tape_model = TapeModel::new(tape_length);

        let writing_tape_capacity =
            input_config.sample_rate.0 as usize * input_config.channels as usize * 10 * 60;

        println!(
            "tape length: {}, bar length: {} seconds, writing tape: {}",
            tape_length, bar_length, writing_tape_capacity
        );

        let input_ring_buffer = RingBuffer::new(RING_BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let (key_sender, key_receiver) = channel();

        let output_ring_buffer = RingBuffer::new(RING_BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let input_stream =
            create_input_stream_live(&input_device, &input_config, tape_length, input_producer);

        let audio_index = Arc::new(AtomicUsize::new(0));

        let output_stream =
            create_output_stream_live(&output_device, &output_config, output_consumer);

        let is_recording = Arc::new(AtomicBool::new(false));
        let is_recording_playback = Arc::new(AtomicBool::new(false));
        let is_play_through = Arc::new(AtomicBool::new(false));
        let sample_averages = Arc::new(Mutex::new([0.0; TAPE_COUNT + 1]));
        let show_beat = Arc::new(AtomicBool::new(false));
        let beat_index = Arc::new(AtomicU32::new(0));

        let mut audio_model: AudioModel = AudioModel {
            tape_length,
            recording_tape,
            tape_model,
            input_consumer,
            key_receiver,
            is_recording: Arc::clone(&is_recording),
            is_recording_playback: Arc::clone(&is_recording_playback),
            is_play_through: Arc::clone(&is_play_through),
            selected_tape: 0,
            output_producer,
            audio_index: Arc::clone(&audio_index),
            writing_tape: Vec::with_capacity(writing_tape_capacity),
            sample_averages: Arc::clone(&sample_averages),
            show_beat: Arc::clone(&show_beat),
            beat_index: Arc::clone(&beat_index),
            metronome: Metronome::new(
                config.bpm,
                input_config.sample_rate.0 * input_config.channels as u32,
            ),
        };

        std::thread::spawn(move || loop {
            audio_model.update();
            std::thread::sleep(Duration::from_micros(1000));
        });

        Modul {
            tape_length,
            input_stream,
            output_stream,
            _time: 0.0,
            audio_index: Arc::clone(&audio_index),
            is_recording: Arc::clone(&is_recording),
            is_recording_playback: Arc::clone(&is_recording_playback),
            is_play_through: Arc::clone(&is_play_through),
            key_sender,
            sample_averages: Arc::clone(&sample_averages),
            _show_beat: Arc::clone(&show_beat),
            beat_index: Arc::clone(&beat_index),
            stats,
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

    pub fn is_play_through(&self) -> bool {
        self.is_play_through.load(Ordering::SeqCst)
    }

    pub fn set_selected_tape(&mut self, selected_tape: usize) {
        self.key_sender
            .send(ModulAction::SelectTape(selected_tape))
            .unwrap();
    }

    pub fn _show_beat(&self) -> bool {
        self._show_beat.load(Ordering::SeqCst)
    }

    pub fn get_beat_index(&self) -> u32 {
        self.beat_index.load(Ordering::SeqCst)
    }

    pub fn switch_metronome(&self, is_active: bool) {
        if is_active {
            self.key_sender.send(ModulAction::StartMetronome).unwrap();
        } else {
            self.key_sender.send(ModulAction::StopMetronome).unwrap();
        }
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

    pub fn play_through(&self) {
        self.key_sender.send(ModulAction::PlayThrough).unwrap();
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

    pub fn get_sample_averages(&self) -> [f32; TAPE_COUNT + 1] {
        *self.sample_averages.lock().unwrap()
    }
}
