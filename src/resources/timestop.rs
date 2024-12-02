use std::time::Duration;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Timestop {
    pub stop_timer: Timer,
    pub cooldown_timer: Timer,

    pub active: bool
}

impl Timestop {
    pub const DURATION: f32 = 3.0;
    pub const COOLDOWN: f32 = 10.0;
}

impl Default for Timestop {
    fn default() -> Self {
        let mut stop_timer = Timer::from_seconds(Self::DURATION, TimerMode::Once);
        let cooldown_timer = Timer::from_seconds(Self::COOLDOWN, TimerMode::Once);

        stop_timer.tick(Duration::from_secs_f32(Self::DURATION));

        Self {
            stop_timer,
            cooldown_timer,

            active: false
        }
    }
}