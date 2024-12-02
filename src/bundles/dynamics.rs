use bevy::prelude::*;
use crate::components::dynamics::*;

#[derive(Bundle, Default)]
pub struct StateBundle {
    pub current_state: CurrentState,
    pub previous_state: PreviousState
}

impl StateBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            current_state: CurrentState {
                position,

                ..default()
            },
            previous_state: PreviousState {
                position
            }
        }
    }
}
