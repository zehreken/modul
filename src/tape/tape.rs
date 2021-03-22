#[derive(Clone)]
pub struct Tape<T> {
    volume: f32,
    pub audio: Vec<T>,
}

impl<T: Clone> Tape<T> {
    pub fn new(default: T, length: usize) -> Self {
        Self {
            volume: 1.0,
            audio: vec![default; length],
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn mute(&mut self) {
        self.volume = 0.0;
    }

    pub fn unmute(&mut self) {
        self.volume = 1.0;
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
}
