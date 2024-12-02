use bevy::prelude::*;

#[derive(Component)]
pub struct Map;

impl Map {
    pub const TOP: f32 = 650.0;
    pub const BOTTOM: f32 = -950.0;
    pub const LEFT: f32 = -1950.0;
    pub const RIGHT: f32 = 1950.0;
}