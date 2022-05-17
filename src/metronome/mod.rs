pub struct Metronome {
    pub is_running: bool,
    beat_count: u32,
    sample_counter: u32,
    tick_period: f32,
    show_beat: bool,
}

impl Metronome {
    pub fn new(bpm: u16, sample_rate: u32) -> Self {
        let tick_period = sample_rate as f32 / (bpm as f32 / 60.0);
        Self {
            is_running: false,
            beat_count: 0,
            sample_counter: 0,
            tick_period,
            show_beat: false,
        }
    }

    pub fn update(&mut self, sample_count: u32) {
        self.sample_counter += sample_count;

        let rem = self.sample_counter % self.tick_period as u32;
        self.show_beat = rem > 0 && rem < 10_000;
        self.beat_count = self.sample_counter / self.tick_period as u32;
    }

    pub fn get_beat_count(&self) -> u32 {
        self.beat_count
    }

    pub fn show_beat(&self) -> bool {
        self.show_beat
    }
}
