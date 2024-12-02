use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CurrentState {
    pub position: Vec2,
    pub velocity: Vec2
}

#[derive(Component, Default)]
pub struct PreviousState {
    pub position: Vec2
}
