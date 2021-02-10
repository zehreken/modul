#[derive(Clone)]
pub struct Tape<T> {
    pub volume: f32,
    pub audio: Vec<T>,
}

impl<T: Clone> Tape<T> {
    pub fn new(default: T, length: usize) -> Self {
        Self {
            volume: 1.0,
            audio: vec![default; length],
        }
    }

    pub fn mute(&mut self) {
        self.volume = 0.0;
    }

    pub fn unmute(&mut self) {
        self.volume = 1.0;
    }

    pub fn clear(&mut self, default: T) {
        for i in 0..self.audio.len() {
            self.audio[i] = default.clone();
        }
    }
}
