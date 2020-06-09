use super::traits::Nannou;
use nannou::draw::Draw;
use nannou::prelude::*;
use std::time::{Duration, Instant};

const MINUTE_IN_MS: u128 = 60000;

pub struct BeatController {
    beat_per_minute: u16,
    pub beat_count: u8,
    pub bar_count: u8,
    pub beat_period: u128,
    time: Instant,
    pub beat_timer: u128,
    pub bar_timer: u128,
    can_draw: bool,
}

impl BeatController {
    pub fn new(beat_per_minute: u16, beat_count: u8, bar_count: u8) -> Self {
        let beat_period = MINUTE_IN_MS / beat_per_minute as u128;
        println!("period: {} ms", beat_period);
        let time = Instant::now();
        Self {
            beat_per_minute,
            beat_count,
            bar_count,
            beat_period,
            time,
            beat_timer: 0,
            bar_timer: 0,
            can_draw: false,
        }
    }
}

impl Nannou for BeatController {
    fn draw(&self, draw: &Draw) {
        if self.can_draw {
            draw.ellipse().w_h(32.0, 32.0).x_y(0.0, 128.0).color(GOLD);
        }
    }

    fn update(&mut self, delta_time: i32) {
        self.can_draw = false;
        let diff: Duration = Instant::now() - self.time;
        self.time = Instant::now();
        // let millis = diff.as_millis();
        let millis = delta_time as u128;
        self.beat_timer += millis;
        self.bar_timer += millis;
        if self.beat_timer >= self.beat_period {
            self.can_draw = true;
            self.beat_timer = 0;
            if self.bar_timer >= self.beat_period * self.beat_count as u128 {
                self.bar_timer = 0;
            }
        }
    }
}
