use crate::metronome::Metronome;
use crate::modul_utils::utils::*;
use crate::tape::Tape;
use ringbuf::{Consumer, Producer};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::mpsc::Receiver;
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
    pub key_receiver: Receiver<ModulAction>,
    pub is_recording: Arc<AtomicBool>,
    pub is_recording_playback: Arc<AtomicBool>,
    pub is_play_through: Arc<AtomicBool>,
    pub selected_tape: usize,
    pub output_producer: Producer<f32>,
    pub audio_index: Arc<AtomicUsize>,
    pub writing_tape: Vec<f32>,
    pub sample_averages: Arc<Mutex<[f32; TAPE_COUNT + 1]>>,
    pub show_beat: Arc<AtomicBool>,
    pub beat_index: Arc<AtomicU32>,
    pub metronome: Metronome,
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

        // while !self.input_consumer.is_empty() {
        while self.input_consumer.len() > 4 {
            let mut audio_index = 0;
            for _ in 0..4 {
                // }
                // for t in self.input_consumer.pop() {
                let t = self.input_consumer.pop().unwrap();
                let t_index = t.index; // this is the cursor(kind of)
                let t_sample = t.sample; // this is the signal that came from the input channel

                if self.is_recording.load(Ordering::SeqCst) {
                    self.recording_tape.push(t);
                }
                // send audio to output
                let mut sample: f32 = 0.0;
                for (tape, average) in self.tape_model.tapes.iter().zip(sample_averages.iter_mut())
                {
                    let tape_sample = tape.audio[t_index] * tape.get_volume();
                    if tape_sample > *average {
                        *average = tape_sample;
                    }
                    sample += tape_sample;
                }

                // sine wave for metronome
                if self.metronome.is_running && self.metronome.show_beat() {
                    const FREQ: f32 = 440.0;
                    sample +=
                        (t_index as f32 * 2.0 * std::f32::consts::PI * FREQ / 44100.0).sin() * 0.05;

                    // println!("t_index: {}, t_sample: {}", t_index, t_sample);
                }
                // ========

                let mut sum = sample;
                if self.is_play_through.load(Ordering::SeqCst) {
                    sum += t_sample;
                    if t_sample > sample_averages[8] {
                        sample_averages[8] = t_sample;
                    }
                }
                // if self.output_producer.len() < 2048 {
                let r = self.output_producer.push(sum);
                // Error checking is probably unnecessary since we have the sleep at the end
                match r {
                    Ok(_) => {}
                    Err(_e) => eprintln!("buffer is full: {}", self.output_producer.len()),
                }
                // }
                if self.is_recording_playback.load(Ordering::SeqCst) {
                    sample += t_sample;
                }
                self.writing_tape.push(sample);

                audio_index = t_index;
            }
            self.audio_index.store(audio_index, Ordering::SeqCst);
        }

        if sample_count > 0 {
            *self.sample_averages.lock().unwrap() = sample_averages;
        }

        self.check_user_input();

        // while self.output_producer.len() >= 2048 {
        //     std::thread::sleep(std::time::Duration::from_millis(1));
        // }
    }

    fn check_user_input(&mut self) {
        for c in self.key_receiver.try_iter() {
            match c {
                ModulAction::Record => {
                    if self.is_recording.load(Ordering::SeqCst) {
                        self.is_recording.store(false, Ordering::SeqCst);

                        let mut audio = vec![0.0; self.tape_length];

                        for t in self.recording_tape.iter() {
                            audio[t.index] = t.sample;
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
                ModulAction::PlayThrough => {
                    self.is_play_through.store(
                        !self.is_play_through.load(Ordering::SeqCst),
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
                _a => {
                    println!("Unhandled action: {:?}", _a);
                }
            }
        }
    }
}
