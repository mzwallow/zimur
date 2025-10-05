use std::time::Instant;

use crate::math::Real;

#[derive(Debug)]
pub struct TimingData {
    pub last_time: Instant,
    // pub frame_number: u32,
    // pub last_frame_timestamp: u32,
    // pub last_frame_duration: u32,
    // pub last_frame_clockstamp: u32,
    // pub last_frame_cloc_ticks: u32,
    // pub is_paused: bool,
    // pub average_frame_duration: u32,
    // pub fps: f32,
}

impl TimingData {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }

    /// Returns the time elapsed in seconds since the last call to `tick()`.
    pub fn tick(&mut self) -> Real {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time);
        self.last_time = current_time;
        Real(delta_time.as_secs_f32())
    }
}
