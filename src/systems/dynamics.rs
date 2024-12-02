use bevy::prelude::*;
use crate::components::dynamics::*;
use crate::components::player::*;
use crate::resources::timestop::*;

pub fn update_states(
    mut state_query: Query<(&mut CurrentState, &mut PreviousState, Option<&Player>)>,
    timestop: Res<Timestop>,
    time: Res<Time>
) {
    for (mut current_state, mut previous_state, player) in &mut state_query {
        if timestop.active && player.is_none() {
            continue;
        }

        let current_state = &mut *current_state; // split borrowing

        previous_state.position = current_state.position;
        current_state.position += current_state.velocity * time.delta_seconds();
    }
}

pub fn interpolate_transforms(
    mut transform_query: Query<(&mut Transform, &CurrentState, &PreviousState, Option<&Player>)>,
    timestop: Res<Timestop>,
    fixed_time: Res<Time<Fixed>>
) {
    for (mut transform, current_state, previous_state, player) in &mut transform_query {
        if timestop.active && player.is_none() {
            continue;
        }

        let alpha = fixed_time.overstep_fraction();
        let interpolated_position = previous_state.position.lerp(current_state.position, alpha);

        transform.translation = interpolated_position.extend(transform.translation.z);
    }
}