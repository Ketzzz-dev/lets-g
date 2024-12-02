use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Enemy {
    pub health: i16,
}

impl Enemy {
    pub const RADIUS: f32 = 45.0;
}

#[derive(Component)]
pub struct Shooter {
    pub shoot_cooldown: Timer,
    pub grace_period: Timer,

    pub shooting: bool,
    pub shoot_direction: Dir2
}

impl Shooter {
    pub const DISTANCE_TO_PLAYER: f32 = 500.0;
    pub const DISTANCE_TO_ATTACK: f32 = 625.0;

    pub const MAX_SPEED: f32 = 375.0;
    pub const ACCELERATION: f32 = 4.5;

    pub const MAX_HEALTH: i16 = 50;

    pub const MIN_SHOOT_COOLDOWN: f32 = 2.5;
    pub const MAX_SHOOT_COOLDOWN: f32 = 5.0;

    pub const GRACE_PERIOD: f32 = 0.25;

    pub fn new() -> Self {
        let shoot_cooldown_duration = thread_rng().gen_range(Self::MIN_SHOOT_COOLDOWN..Self::MAX_SHOOT_COOLDOWN);

        Self {
            shoot_cooldown: Timer::from_seconds(shoot_cooldown_duration, TimerMode::Once),
            grace_period: Timer::from_seconds(Self::GRACE_PERIOD, TimerMode::Once),

            shooting: false,
            shoot_direction: Dir2::X
        }
    }
}

#[derive(Component)]
pub struct Striker {
    pub strike_cooldown: Timer,
    pub grace_period: Timer,

    pub striking: bool,
    pub strike_direction: Dir2
}

impl Striker {
    pub const DISTANCE_TO_ATTACK: f32 = 200.0;

    pub const MAX_SPEED: f32 = 500.0;
    pub const ACCELERATION: f32 = 3.5;

    pub const MAX_HEALTH: i16 = 75;

    pub const ATTACK_COOLDOWN: f32 = 1.5;
    pub const GRACE_PERIOD: f32 = 0.25;

    pub fn new() -> Self {
        Self {
            strike_cooldown: Timer::from_seconds(Self::ATTACK_COOLDOWN, TimerMode::Once),
            grace_period: Timer::from_seconds(Self::GRACE_PERIOD, TimerMode::Once),

            striking: false,
            strike_direction: Dir2::X
        }
    }
}

#[derive(Component)]
pub struct EnemyAnimationController {
    pub timer: Timer,
    pub run_frame_index: u32
}

impl EnemyAnimationController {
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