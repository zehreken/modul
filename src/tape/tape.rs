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

    // Don't use this, very bad for memory
    pub fn clear(&mut self) {
        // Clear audio vector
        self.audio.clear();
    }
}
