use std::ops::{Add, AddAssign};

#[derive(Clone)]
pub struct Tape<T> {
    volume: f32,
    is_muted: bool,
    is_solo: bool,
    pub audio: Vec<T>,
}

impl<T: Copy + Clone + Add + AddAssign> Tape<T> {
    pub fn new(default: T, length: usize) -> Self {
        Self {
            volume: 1.0,
            is_muted: false,
            is_solo: false,
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

    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    pub fn toggle_solo(&mut self) {
        self.is_solo = !self.is_solo;
    }

    pub fn is_solo(&self) -> bool {
        self.is_solo
    }

    pub fn volume_up(&mut self) {
        if self.volume < 1.0 {
            self.volume += 0.01;
        }
    }

    pub fn volume_down(&mut self) {
        if self.volume > 0.01 {
            self.volume -= 0.01;
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
