use super::super::Config;
use super::audio_model::*;
use super::utils::*;
use crate::metronome::Metronome;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, Stream, StreamConfig};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};
use std::collections::VecDeque;
use std::time::Duration;

pub struct Stats {
    pub bpm: u16,
    pub bar_count: usize,
    pub bar_length: f32,
    pub input_device_name: String,
    pub input_channel_count: u16,
    pub input_buffer_size: u32,
    pub output_channel_count: u16,
    pub output_buffer_size: u32,
    pub output_device_name: String,
}

pub struct Modul {
    pub tape_length: usize,
    _input_stream: Stream,
    _output_stream: Stream,
    _time: f32,
    audio_index: usize,
    action_producer: HeapProducer<ModulAction>,
    _modul_message_producer: HeapProducer<ModulMessage>,
    modul_message_consumer: HeapConsumer<ModulMessage>,
    is_recording: bool,
    is_recording_playback: bool,
    is_play_through: bool,
    sample_averages: [f32; TAPE_COUNT + 1],
    pub samples_for_graphs: [[f32; SAMPLE_GRAPH_SIZE]; TAPE_COUNT],
    _show_beat: bool,
    beat_index: u32,
    pub stats: Stats,
    pub message_history: VecDeque<String>,
    log_consumer: HeapConsumer<String>,
    pub instant: std::time::Instant,
}

impl Modul {
    pub fn new(config: &Config) -> Self {
        let host = cpal::default_host();

        let input_device = host.default_input_device().unwrap();
        let output_device = host.default_output_device().unwrap();

        let mut input_config: StreamConfig = input_device.default_input_config().unwrap().into();
        println!("input channel count: {}", input_config.channels);
        println!("input sample rate: {:?}", input_config.sample_rate);

        let beats = 4.0; // This corresponds to the time, at the moment it is 4/4
        let seconds_per_beat = 60.0 / config.bpm as f32;
        let bar_length = beats * seconds_per_beat; // bar length in seconds, beats * seconds per beat(60.0 / BPM)

        let output_config: StreamConfig = output_device.default_output_config().unwrap().into();
        println!("output channel count: {}", output_config.channels);
        println!("output sample rate: {:?}", output_config.sample_rate);

        /*
        ATTENTION:
        If buffer capacity and update frequency is related, if update frequency is low
        then the buffer will not be emptied fast enough and some input will be lost
        This is unnecesary since I don't push to the buffer if buffer.len() is 2048
        */
        const BUFFER_SIZE: u32 = 128; // Suggested buffer size for recording is 128, in my tests even 32 works fine
        const RING_BUFFER_CAPACITY: usize = 8192;

        let message_history = VecDeque::with_capacity(10);
        input_config.buffer_size = BufferSize::Fixed(BUFFER_SIZE);

        let output_buffer_size = match output_device
            .default_output_config()
            .unwrap()
            .config()
            .buffer_size
        {
            BufferSize::Default => 512,
            BufferSize::Fixed(v) => v,
        };
        let stats = Stats {
            bpm: config.bpm,
            bar_count: config.bar_count,
            bar_length,
            input_device_name: input_device.name().unwrap(),
            input_channel_count: input_config.channels,
            input_buffer_size: BUFFER_SIZE,
            output_device_name: output_device.name().unwrap(),
            output_channel_count: output_config.channels,
            output_buffer_size,
        };

        // sample rate * channel count(4 on personal mac) * bar length in seconds * bar count
        let mut tape_length: usize = (input_config.sample_rate.0 as f32
            * input_config.channels as f32
            * bar_length
            * config.bar_count as f32) as usize;
        tape_length -= tape_length % input_config.channels as usize;

        let tape_model = TapeModel::new(tape_length);

        let ten_minutes_in_seconds = 10 * 60;
        let preallocated_capacity = input_config.sample_rate.0 as usize
            * input_config.channels as usize
            * ten_minutes_in_seconds;

        println!(
            "tape length: {}, bar length: {} seconds, writing tape: {}",
            tape_length, bar_length, preallocated_capacity
        );

        let message_buffer: HeapRb<String> = HeapRb::new(10);
        let (message_producer, message_consumer) = message_buffer.split();

        let input_ring_buffer = HeapRb::new(RING_BUFFER_CAPACITY);
        let (input_producer, input_consumer) = input_ring_buffer.split();

        let action_ring_buffer: HeapRb<ModulAction> = HeapRb::new(16);
        let (action_producer, action_consumer) = action_ring_buffer.split();

        let message_ring_buffer_to: HeapRb<ModulMessage> = HeapRb::new(2_usize.pow(10));
        let (modul_message_producer, audio_message_consumer) = message_ring_buffer_to.split();

        let message_ring_buffer_from: HeapRb<ModulMessage> = HeapRb::new(2_usize.pow(10));
        let (audio_message_producer, modul_message_consumer) = message_ring_buffer_from.split();

        let output_ring_buffer = HeapRb::new(RING_BUFFER_CAPACITY);
        let (output_producer, output_consumer) = output_ring_buffer.split();

        let input_stream =
            create_input_stream_live(&input_device, &input_config, tape_length, input_producer);

        let audio_index = 0;

        let output_stream =
            create_output_stream_live(&output_device, &output_config, output_consumer);

        let sample_averages = [0.0; TAPE_COUNT + 1];
        let samples_for_graphs = [[0.0; SAMPLE_GRAPH_SIZE]; TAPE_COUNT];
        let show_beat = false;
        let beat_index = 0;

        let mut audio_model: AudioModel = AudioModel {
            tape_length,
            recording_tape: Vec::with_capacity(preallocated_capacity),
            tape_model,
            input_consumer,
            action_consumer,
            audio_message_producer,
            audio_message_consumer,
            is_recording: false,
            is_recording_playback: false,
            is_play_through: false,
            audio_index,
            primary_tape: 0,
            secondary_tapes: [false; TAPE_COUNT],
            output_producer,
            writing_tape: Vec::with_capacity(preallocated_capacity),
            sample_averages,
            samples_for_graphs,
            show_beat,
            beat_index,
            metronome: Metronome::new(
                config.bpm,
                input_config.sample_rate.0,
                input_config.channels as u32,
            ),
            output_channel_count: output_config.channels as usize,
            log_producer: message_producer,
        };

        std::thread::spawn(move || loop {
            audio_model.update();
            std::thread::sleep(Duration::from_micros(1000));
        });

        Modul {
            tape_length,
            _input_stream: input_stream,
            _output_stream: output_stream,
            _time: 0.0,
            audio_index,
            is_recording: false,
            is_recording_playback: false,
            is_play_through: false,
            action_producer,
            _modul_message_producer: modul_message_producer,
            modul_message_consumer,
            sample_averages,
            samples_for_graphs,
            _show_beat: show_beat,
            beat_index,
            stats,
            message_history,
            log_consumer: message_consumer,
            instant: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self) {
        while !self.modul_message_consumer.is_empty() {
            let message = self.modul_message_consumer.pop().unwrap();
            match message {
                ModulMessage::AudioIndex(audio_index) => self.audio_index = audio_index,
                ModulMessage::Recording(is_recording) => self.is_recording = is_recording,
                ModulMessage::RecordingPlayback(is_recording_playback) => {
                    self.is_recording_playback = is_recording_playback
                }
                ModulMessage::PlayThrough(is_play_through) => {
                    self.is_play_through = is_play_through
                }
                ModulMessage::ShowBeat(show_beat) => self._show_beat = show_beat,
                ModulMessage::BeatIndex(beat_index) => self.beat_index = beat_index,
                ModulMessage::SampleAverages(sample_averages) => {
                    self.sample_averages = sample_averages
                }
                ModulMessage::SamplesForGraphs(samples_for_graphs) => {
                    self.samples_for_graphs = samples_for_graphs
                }
            }
        }
        while !self.log_consumer.is_empty() {
            let message = self.log_consumer.pop().unwrap();
            self.add_message(message);
        }
    }

    pub fn add_message(&mut self, message: String) {
        let message = format!("[{:5.2}] {}", self.instant.elapsed().as_secs_f32(), message);
        if self.message_history.len() == 10 {
            self.message_history.pop_front();
        }
        self.message_history.push_back(message);
    }

    pub fn _get_time(&self) -> f32 {
        self._time
    }

    pub fn get_audio_index(&self) -> usize {
        self.audio_index
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn is_recording_playback(&self) -> bool {
        self.is_recording_playback
    }

    pub fn is_play_through(&self) -> bool {
        self.is_play_through
    }

    pub fn select_primary_tape(&mut self, primary_tape: usize) {
        self.action_producer
            .push(ModulAction::SelectPrimaryTape(primary_tape))
            .unwrap();
    }

    pub fn select_secondary_tape(&mut self, secondary_tape: usize) {
        self.action_producer
            .push(ModulAction::SelectSecondaryTape(secondary_tape))
            .unwrap();
    }

    pub fn merge_tapes(&mut self) {
        self.action_producer.push(ModulAction::MergeTapes).unwrap();
    }

    pub fn _show_beat(&self) -> bool {
        self._show_beat
    }

    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    pub fn switch_metronome(&mut self, is_active: bool) {
        if is_active {
            self.action_producer
                .push(ModulAction::StartMetronome)
                .unwrap();
        } else {
            self.action_producer
                .push(ModulAction::StopMetronome)
                .unwrap();
        }
    }

    pub fn record(&mut self) {
        self.action_producer.push(ModulAction::Record).unwrap();
    }

    pub fn record_playback(&mut self) {
        self.action_producer.push(ModulAction::Playback).unwrap();
    }

    pub fn play_through(&mut self) {
        self.action_producer.push(ModulAction::PlayThrough).unwrap();
    }

    pub fn write(&mut self) {
        self.action_producer.push(ModulAction::Write).unwrap();
    }

    pub fn clear_all(&mut self) {
        self.action_producer.push(ModulAction::ClearAll).unwrap();
    }

    pub fn clear(&mut self) {
        self.action_producer.push(ModulAction::Clear).unwrap();
    }

    pub fn toggle_mute(&mut self) {
        self.action_producer.push(ModulAction::ToggleMute).unwrap();
    }

    pub fn toggle_solo(&mut self) {
        self.action_producer.push(ModulAction::ToggleSolo).unwrap();
    }

    pub fn volume_up(&mut self) {
        self.action_producer.push(ModulAction::VolumeUp).unwrap();
    }

    pub fn volume_down(&mut self) {
        self.action_producer.push(ModulAction::VolumeDown).unwrap();
    }

    pub fn get_sample_averages(&self) -> [f32; TAPE_COUNT + 1] {
        self.sample_averages
    }
}
