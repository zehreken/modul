use super::traits::Nannou;
use nannou::draw::Draw;
use nannou::prelude::*;

const SAMPLES_PER_MINUTE:usize = super::modul::SAMPLE_RATE * 60;

pub struct BeatController {
    beat_per_minute: u16,
    pub beat_count: u8,
    pub bar_count: u8,
    pub beat_period: u128,
    pub beat_timer: u128,
    pub bar_timer: u128,
    can_draw: bool,
}

impl BeatController {
    pub fn new(beat_per_minute: u16, beat_count: u8, bar_count: u8) -> Self {
        let beat_period = SAMPLES_PER_MINUTE as u128 / beat_per_minute as u128;
        println!("period: {} samples", beat_period);
        Self {
            beat_per_minute,
            beat_count,
            bar_count,
            beat_period,
            beat_timer: 0,
            bar_timer: 0,
            can_draw: false,
        }
    }
}

impl Nannou for BeatController {
    fn draw(&self, draw: &Draw) {
        if self.can_draw {
            draw.rect().w_h(1024.0, 20.0).x_y(0.0, 224.0).color(GOLD);
        }
    }

    fn update(&mut self, delta_time: i32) {
        self.can_draw = false;
        let samples = delta_time as u128;
        self.beat_timer += samples;
        self.bar_timer += samples;

        // println!("{}, {}, {}", samples, self.beat_timer, self.bar_timer);
        if self.beat_timer >= self.beat_period {
            self.can_draw = true;
            self.beat_timer = 0;
            if self.bar_timer >= self.beat_period * self.beat_count as u128 {
                self.bar_timer = 0;
            }
        }
    }
}
