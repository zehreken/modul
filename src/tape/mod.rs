use std::ops::{Add, AddAssign};

#[derive(Clone)]
pub struct Tape<T> {
    volume: f32,
    is_muted: bool,
    pub audio: Vec<T>,
}

impl<T: Copy + Clone + Add + AddAssign> Tape<T> {
    pub fn new(default: T, length: usize) -> Self {
        Self {
            volume: 1.0,
            is_muted: false,
            audio: vec![default; length],
        }
    }

    pub fn get_volume(&self) -> f32 {
        if self.is_muted {
            0.0
        } else {
            self.volume
        }
    }

    pub fn mute(&mut self) {
        self.is_muted = true;
    }

    pub fn unmute(&mut self) {
        self.is_muted = false;
    }

    pub fn volume_up(&mut self) {
        if self.volume < 1.0 {
            self.volume += 0.05;
        }
    }

    pub fn volume_down(&mut self) {
        if self.volume > 0.0 {
            self.volume -= 0.05;
        }
    }

    pub fn clear(&mut self, default: T) {
        for i in 0..self.audio.len() {
            self.audio[i] = default.clone();
        }
    }

    pub fn add(&mut self, other: Vec<T>) {
        for i in 0..self.audio.len() {
            self.audio[i] += other[i];
        }
    }
}
