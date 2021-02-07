use std::time::Instant;

pub struct BeatController {
    instant: Instant,
    beat_per_minute: u16,
    pub beat_count: u8,
    pub bar_count: u8,
    pub beat_period: u32,
    pub beat_timer: u32,
    pub bar_timer: u32,
    can_draw: bool,
}

impl BeatController {
    pub fn new(sample_rate: u32, beat_per_minute: u16, beat_count: u8, bar_count: u8) -> Self {
        let beat_period = (sample_rate * 60) / beat_per_minute as u32;
        println!("period: {} samples", beat_period);
        Self {
            instant: Instant::now(),
            beat_per_minute,
            beat_count,
            bar_count,
            beat_period,
            beat_timer: 0,
            bar_timer: 0,
            can_draw: false,
        }
    }

    pub fn update(&mut self, delta_samples: u32) {
        self.can_draw = false;
        let samples = delta_samples;
        self.beat_timer += samples;
        self.bar_timer += samples;

        if self.beat_timer >= self.beat_period {
            self.can_draw = true;
            self.beat_timer = 0;
            if self.bar_timer >= self.beat_period * self.beat_count as u32 {
                self.bar_timer = 0;
            }
        }
    }
}
