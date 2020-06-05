use super::traits::Nannou;
use nannou::draw::Draw;
use nannou::prelude::*;
use std::time::{Duration, Instant};

const MINUTE_IN_MS: u128 = 60000;

pub struct BeatController {
    bpm: u16,
    period: u128,
    time: Instant,
    counter: u128,
    can_draw: bool,
}

impl BeatController {
    pub fn new(bpm: u16) -> Self {
        let period = MINUTE_IN_MS / bpm as u128;
        let time = Instant::now();
        Self {
            bpm,
            period,
            time,
            counter: 0,
            can_draw: false,
        }
    }
}

impl Nannou for BeatController {
    fn draw(&self, draw: &Draw) {
        if self.can_draw {
            draw.ellipse().w_h(300.0, 300.0).x_y(0.0, 0.0).color(GOLD);
        }
    }

    fn update(&mut self) {
        self.can_draw = false;
        let diff: Duration = Instant::now() - self.time;
        self.time = Instant::now();
        self.counter += diff.as_millis();
        if self.counter > self.period {
            self.can_draw = true;
            self.counter = 0;
        }
    }
}
