pub struct Metronome {
    pub is_running: bool,
}

impl Metronome {
    pub fn new() -> Self {
        Self { is_running: false }
    }
}
