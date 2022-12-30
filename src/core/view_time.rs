use std::time::Instant;

pub struct ViewTime {
    start: Instant,
}

impl ViewTime {
    pub fn new() -> Self {
        ViewTime {
            start: Instant::now(),
        }
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let delta_time = now.duration_since(self.start);
        self.start = now;

        delta_time.as_secs_f32()
    }
}
