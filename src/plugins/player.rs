use bevy::prelude::*;
use crate::AppState;
use crate::systems::dynamics::*;
use crate::systems::player::*;
use crate::systems::world_cursor::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            .add_systems(FixedUpdate, (
                regenerate_player,

                move_player
                    .before(update_states),
                (
                    confine_player_bounds,
                    damage_player
                ).after(update_states),
            ).run_if(in_state(AppState::Game)))
            .add_systems(Update, (
                reload_player,
                animate_player,
                stop_time,

                (shoot_player, point_player_gun_to_cursor)
                    .after(update_world_cursor)
            ).run_if(in_state(AppState::Game)));
    }
}