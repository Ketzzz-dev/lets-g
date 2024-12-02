use std::time::Duration;
use bevy::prelude::*;

#[derive(Resource)]
pub struct WaveSystem {
    pub timer: Timer,

    pub wave: u32,
    pub max_enemies: u32
}

impl WaveSystem {
    pub const WAVE_TIME: f32 = 30.0;

    pub const ENEMIES_PER_WAVE: u32 = 2;
    pub const STARTING_ENEMIES: u32 = 6;
}

impl Default for WaveSystem {
    fn default() -> Self {
        let mut timer = Timer::new(Duration::from_secs_f32(Self::WAVE_TIME), TimerMode::Repeating);

        timer.tick(Duration::from_secs_f32(Self::WAVE_TIME - 5.0));

        Self {
            timer,

            wave: 0,
            max_enemies: Self::STARTING_ENEMIES - Self::ENEMIES_PER_WAVE
        }
    }
}