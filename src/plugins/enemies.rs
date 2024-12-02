use bevy::prelude::*;
use crate::AppState;
use crate::systems::dynamics::*;
use crate::systems::enemies::*;
use crate::systems::player::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Game), despawn_enemies)
            .add_systems(FixedUpdate, (
                shoot_shooters,
                strike_strikers,
                spawn_wave,

                (move_shooters, move_strikers)
                    .before(update_states),

                (space_enemies, damage_enemies, confine_enemy_bounds)
                    .after(update_states)
            ).run_if(in_state(AppState::Game)).run_if(time_paused))
            .add_systems(Update, (
                animate_enemies,
                point_shooter_gun_to_player,
                point_striker_knife_to_player
            ).run_if(in_state(AppState::Game)).run_if(time_paused));
    }
}