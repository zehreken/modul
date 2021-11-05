use crate::modul_utils::utils::*;
use crate::tape::tape::Tape;
use ringbuf::{Consumer, Producer};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{atomic::AtomicBool, mpsc::Receiver};
use std::sync::{Arc, Mutex};

pub struct TapeModel {
    pub tapes: [Tape<f32>; 8],
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
    pub selected_tape: usize,
    pub output_producer: Producer<f32>,
    pub audio_index: Arc<AtomicUsize>,
    pub writing_tape: Vec<f32>,
    pub sample_averages: Arc<Mutex<[f32; 8]>>,
}

pub struct Input {
    pub index: usize,
    pub sample: f32,
}

impl AudioModel {
    pub fn update(&mut self) {
        let mut sample_averages = [0.0; 8];
        let sample_count = self.input_consumer.len();
        while !self.input_consumer.is_empty() {
            let mut audio_index = 0;
            for t in self.input_consumer.pop() {
                let t_index = t.index;
                let t_sample = t.sample;
                if self.is_recording.load(Ordering::SeqCst) {
                    self.recording_tape.push(t);
                }
                // send audio to output
                let mut sample: f32 = 0.0;
                for (tape, average) in self.tape_model.tapes.iter().zip(sample_averages.iter_mut())
                {
                    *average += tape.audio[t_index] * tape.get_volume();
                    sample += tape.audio[t_index] * tape.get_volume();
                }

                let r = self.output_producer.push(sample + t_sample);
                match r {
                    Ok(_) => {}
                    Err(_e) => eprintln!("error: {}", self.output_producer.len()),
                }
                if self.is_recording_playback.load(Ordering::SeqCst) {
                    sample += t_sample;
                }
                self.writing_tape.push(sample);

                audio_index = t_index;
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
