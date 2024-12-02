use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub ammo: u32,
    pub health: f32,

    pub shoot_cooldown: Timer,
    pub reload_timer: Timer,
}

impl Player {
    pub const MAX_SPEED: f32 = 550.0;
    pub const ACCELERATION: f32 = 6.0;
    pub const DECELERATION: f32 = 12.0;

    pub const MAX_AMMO: u32 = 6;
    pub const MAX_HEALTH: f32 = 100.0;

    pub const RADIUS: f32 = 40.0;

    pub const SHOOT_COOLDOWN: f32 = 1.0 / 6.0;
    pub const RELOAD_DURATION: f32 = 0.75;

    pub fn new() -> Self {
        Self {
            ammo: Self::MAX_AMMO,
            health: Self::MAX_HEALTH,

            shoot_cooldown: Timer::from_seconds(Self::SHOOT_COOLDOWN, TimerMode::Once),
            reload_timer: Timer::from_seconds(Self::RELOAD_DURATION, TimerMode::Once)
        }
    }
}

#[derive(Component)]
pub struct PlayerAnimationController {
    pub timer: Timer,
    pub run_frame_index: u32
}

impl PlayerAnimationController {
    pub const ANIMATION_FPS: f32 = 1.0 / 12.0;

    pub const ATLAS_TILE_SIZE: UVec2 = UVec2::splat(128);
    pub const ATLAS_COLUMNS: u32 = 16;
    pub const ATLAS_ROWS: u32 = 9;

    pub const DIRECTION_INDICES: [u32; 8] = [
        4, // e
        7, // ne
        5, // n
        6, // nw
        3, // w
        1, // sw
        0, // s
        2 // se
    ];

    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(Self::ANIMATION_FPS, TimerMode::Repeating),
            run_frame_index: 0
        }
    }
}