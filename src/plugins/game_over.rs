use bevy::prelude::*;
use crate::AppState;
use crate::systems::game_over::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over)
            .add_systems(Update, tick_game_over.run_if(in_state(AppState::GameOver)));
    }
}