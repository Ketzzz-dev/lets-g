use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub owner: Entity
}

impl Bullet {
    pub const MAX_SPEED: f32 = 5000.0;
}