use crate::metronome::Metronome;
use crate::modul_utils::utils;
use crate::modul_utils::utils::*;
use crate::tape::Tape;
use ringbuf::{Consumer, Producer};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub struct TapeModel {
    pub tapes: [Tape<f32>; TAPE_COUNT],
}

impl TapeModel {
    pub fn new(length: usize) -> Self {
        let tapes: [Tape<f32>; TAPE_COUNT] =
            array_init::array_init(|_| Tape::<f32>::new(0.0, length));

        Self { tapes }
    }
}

/// Used to transfer data to the audio thread
/// In this context audio thread is the thread that communicates
/// with input and output streams(each has its own thread)
pub struct AudioModel {
    pub tape_length: usize,
    pub recording_tape: Vec<Input>,
    pub tape_model: TapeModel,
    pub input_consumer: Consumer<Input>,
    pub action_consumer: Consumer<ModulAction>,
    pub audio_message_producer: Producer<ModulMessage>,
    pub audio_message_consumer: Consumer<ModulMessage>,
    pub is_recording: bool,
    pub is_recording_playback: bool,
    pub is_play_through: bool,
    pub audio_index: usize,
    pub selected_tape: usize,
    pub secondary_tapes: [bool; TAPE_COUNT],
    pub output_producer: Producer<f32>,
    pub writing_tape: Vec<f32>,
    pub sample_averages: Arc<Mutex<[f32; TAPE_COUNT + 1]>>,
    pub samples_for_graphs: Arc<Mutex<[[f32; SAMPLE_GRAPH_SIZE]; TAPE_COUNT]>>,
    pub show_beat: Arc<AtomicBool>,
    pub beat_index: Arc<AtomicU32>,
    pub metronome: Metronome,
    pub output_channel_count: usize,
    pub log_producer: Producer<String>,
}

pub struct Input {
    pub index: usize,
    pub sample: f32,
}

impl AudioModel {
    pub fn update(&mut self) {
        let mut sample_averages = [0.0; TAPE_COUNT + 1];
        let sample_count = self.input_consumer.len();

        // Update metronome
        self.metronome.update(sample_count as u32);

        self.show_beat
            .store(self.metronome.show_beat(), Ordering::SeqCst);
        self.beat_index
            .store(self.metronome.get_beat_index(), Ordering::SeqCst);

        while !self.input_consumer.is_empty() {
            let t = self.input_consumer.pop().unwrap();
            let t_index = t.index; // this is the cursor(kind of)
            self.audio_index = t_index;
            let t_sample = t.sample; // this is the signal that came from the input channel

            if self.is_recording {
                self.recording_tape.push(t);
            }

            // send audio to output
            let mut sample: f32 = 0.0;
            for (tape, average) in self.tape_model.tapes.iter().zip(sample_averages.iter_mut()) {
                let tape_sample = tape.audio[t_index] * tape.get_volume();
                if tape_sample > *average {
                    *average = tape_sample;
                }
                sample += tape_sample;
            }

            let mut sum = sample;
            if self.is_play_through {
                sum += t_sample;
                if t_sample > sample_averages[8] {
                    sample_averages[8] = t_sample;
                }
            }

            // sine wave for metronome
            if self.metronome.is_running && self.metronome.show_beat() {
                let first_beat = self.metronome.get_beat_index() % 4 == 0;
                let freq: f32 = if first_beat {
                    utils::C_FREQ
                } else {
                    utils::A_FREQ
                };
                let volume = 0.2;
                sum +=
                    (t_index as f32 * 2.0 * std::f32::consts::PI * freq / 44100.0).sin() * volume;

                // println!("t_index: {}, t_sample: {}", t_index, t_sample);
            }
            // ========

            let r = self.output_producer.push(sum);
            match r {
                Ok(_) => {}
                Err(_e) => {
                    self.log_producer
                        .push(format!("buffer is full: {}", self.output_producer.len()))
                        .unwrap();
                }
            }
            if self.is_recording_playback {
                sample += t_sample;
            }
            self.writing_tape.push(sample);
        }

        self.audio_message_producer
            .push(ModulMessage::AudioIndex(self.audio_index))
            .expect("audio message buffer is full");

        // This is to prevent left/right switching
        let output_channel_count = self.output_channel_count;
        if self.output_producer.len() % output_channel_count != 0 {
            self.log_producer
                .push(format!(
                    "output_producer.len % {} is not 0, fixing",
                    output_channel_count
                ))
                .unwrap();
            self.output_producer.push(0.0).unwrap();
        }

        if sample_count > 0 {
            *self.sample_averages.lock().unwrap() = sample_averages;
        }

        self.check_user_input();
    }

    fn update_waveform(&self, id: usize, audio: &Vec<f32>) {
        let mut counter = 0;
        let mut temp = vec![];
        let mut sum = 0.0;
        // Adding 1 to make sure that we have samples more than SAMPLE_GRAPH_SIZE
        // Be my guest if you find a smarter way to do that
        let size = audio.len() / (SAMPLE_GRAPH_SIZE + 1);
        for sample in audio {
            if *sample < 0.0 {
                sum -= *sample
            } else {
                sum += *sample
            };
            if counter >= size {
                temp.push(sum);
                sum = 0.0;
                counter = 0;
            }
            counter += 1;
        }
        self.samples_for_graphs.lock().unwrap()[id] = temp[0..SAMPLE_GRAPH_SIZE]
            .try_into()
            .expect("Error setting samples_for_graphs");
    }

    fn check_user_input(&mut self) {
        while !self.action_consumer.is_empty() {
            let action = self.action_consumer.pop().unwrap();
            match action {
                ModulAction::Record => {
                    if self.is_recording {
                        self.is_recording = false;
                        self.audio_message_producer
                            .push(ModulMessage::Recording(self.is_recording))
                            .unwrap();

                        let mut audio = vec![0.0; self.tape_length];

                        for t in self.recording_tape.iter() {
                            audio[t.index] = t.sample;
                        }

                        self.update_waveform(self.selected_tape, &audio);

                        self.tape_model.tapes[self.selected_tape].audio = audio;
                        self.recording_tape.clear();
                    } else {
                        self.is_recording = true;
                        self.audio_message_producer
                            .push(ModulMessage::Recording(self.is_recording))
                            .unwrap();
                        self.recording_tape.clear();
                    }
                }
                ModulAction::Playback => {
                    self.is_recording_playback = !self.is_recording_playback;
                    self.audio_message_producer
                        .push(ModulMessage::RecordingPlayback(self.is_recording_playback))
                        .unwrap();
                }
                ModulAction::PlayThrough => {
                    self.is_play_through = !self.is_play_through;
                    self.audio_message_producer
                        .push(ModulMessage::PlayThrough(self.is_play_through))
                        .unwrap();
                }
                ModulAction::Write => {
                    // let tape = merge_tapes(&self.tape_model.tapes);
                    // write_tape(&tape, "test");
                    write(&self.writing_tape, "full");
                }
                ModulAction::ClearAll => {
                    self.log_producer
                        .push("Cleared all tapes".to_owned())
                        .unwrap();
                    for id in 0..TAPE_COUNT {
                        self.tape_model.tapes[id].clear(0.0);
                        self.update_waveform(id, &vec![0.0; SAMPLE_GRAPH_SIZE]);
                    }
                }
                ModulAction::Clear => {
                    self.log_producer
                        .push(format!("Cleared tape {}", self.selected_tape + 1))
                        .unwrap();
                    self.tape_model.tapes[self.selected_tape].clear(0.0);
                    self.update_waveform(self.selected_tape, &vec![0.0; SAMPLE_GRAPH_SIZE]);
                }
                ModulAction::Mute => {
                    self.tape_model.tapes[self.selected_tape].mute();
                }
                ModulAction::Unmute => {
                    self.tape_model.tapes[self.selected_tape].unmute();
                }
                ModulAction::SelectTape(tape) => {
                    self.selected_tape = tape.clamp(0, TAPE_COUNT);
                }
                ModulAction::VolumeUp => {
                    self.tape_model.tapes[self.selected_tape].volume_up();
                }
                ModulAction::VolumeDown => {
                    self.tape_model.tapes[self.selected_tape].volume_down();
                }
                ModulAction::StartMetronome => {
                    self.metronome.is_running = true;
                }
                ModulAction::StopMetronome => {
                    self.metronome.is_running = false;
                }
            }
        }
    }
}
