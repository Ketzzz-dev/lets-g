use std::f32::consts::PI;
use bevy::prelude::*;
use crate::bundles::dynamics::*;
use crate::components::bullet::*;
use crate::components::dynamics::*;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub sprite_bundle: SpriteBundle,
    pub state_bundle: StateBundle
}

impl BulletBundle {
    pub fn new(direction: Dir2, owner: Entity, position: Vec2, texture: Handle<Image>, color: Color) -> Self {
        Self {
            bullet: Bullet { owner },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position.extend(-1.0))
                    .with_rotation(Quat::from_rotation_z(direction.to_angle() + PI / 2.0)),
                texture,
                sprite: Sprite {
                    color,

                    ..default()
                },

                ..default()
            },
            state_bundle: StateBundle {
                current_state: CurrentState {
                    position,
                    velocity: direction * Bullet::MAX_SPEED
                },
                previous_state: PreviousState {
                    position
                }
            }
        }
    }
}