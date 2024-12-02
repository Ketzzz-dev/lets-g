use bevy::prelude::*;
use crate::AppState;
use crate::systems::dynamics::*;

pub struct DynamicsPlugin;

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, update_states
                .run_if(in_state(AppState::Game)
            ))
            .add_systems(Update, interpolate_transforms.run_if(in_state(AppState::Game)));
    }
}