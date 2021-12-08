pub struct Metronome {
    pub is_running: bool,
    bpm: u16,
}

impl Metronome {
    pub fn new(bpm: u16) -> Self {
        Self {
            is_running: false,
            bpm,
        }
    }

    pub fn update(&self) {
        if self.is_running {}
    }
}
