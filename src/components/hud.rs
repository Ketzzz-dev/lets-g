use bevy::prelude::*;

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct Ammo;

impl Ammo {
    pub const ATLAS_TILE_SIZE: UVec2 = UVec2::splat(600);
    pub const ATLAS_COLUMNS: u32 = 3;
    pub const ATLAS_ROWS: u32 = 3;

    pub const AMMO_INDICES: [usize; 7] = [
        0,
        3,
        6,
        1,
        2,
        4,
        7
    ];
}

#[derive(Component)]
pub struct Hourglass;

impl Hourglass {
    pub const ATLAS_TILE_SIZE: UVec2 = UVec2::splat(600);
    pub const ATLAS_COLUMNS: u32 = 26;
    pub const ATLAS_ROWS: u32 = 15;

    pub const FRAMES: usize = 372;
}

#[derive(Component)]
pub struct Stopwatch;

#[derive(Component)]
pub struct Health;