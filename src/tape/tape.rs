#[derive(Clone)]
pub struct Tape {
    pub volume: f32,
    pub audio: Vec<[f32; 2]>,
}

impl Tape {
    pub fn new(length: usize) -> Self {
        Self {
            volume: 1.0,
            audio: vec![[0.0; 2]; length],
        }
    }

    pub fn mute(&mut self) {
        self.volume = 0.0;
    }

    pub fn clear(&mut self) {
        // Clear audio vector
    }
}
