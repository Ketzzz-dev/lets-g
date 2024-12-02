use bevy::prelude::*;

#[derive(Component)]
pub struct GameOver {
    pub timer: Timer
}

impl GameOver {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once)
        }
    }
}