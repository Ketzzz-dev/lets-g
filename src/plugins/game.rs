use bevy::prelude::*;
use crate::AppState;
use crate::events::game_over::*;
use crate::plugins::dynamics::*;
use crate::plugins::enemies::*;
use crate::plugins::hud::*;
use crate::plugins::player::*;
use crate::resources::enemies::*;
use crate::resources::timestop::*;
use crate::systems::camera::*;
use crate::systems::dynamics::*;
use crate::systems::game_over::*;
use crate::systems::map::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HudPlugin,
                DynamicsPlugin,
                PlayerPlugin,
                EnemiesPlugin
            ))
            .init_resource::<WaveSystem>()
            .init_resource::<Timestop>()
            .add_event::<GameOverEvent>()
            .add_systems(OnEnter(AppState::Game), spawn_map)
            .add_systems(OnExit(AppState::Game), despawn_map)
            .add_systems(Update, (
                check_game_over,

                follow_player
                    .after(interpolate_transforms),
                confine_camera_bounds
                    .after(follow_player)
            ).run_if(in_state(AppState::Game)));
    }
}