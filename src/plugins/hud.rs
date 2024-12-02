use bevy::prelude::*;
use crate::AppState;
use crate::systems::hud::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(OnExit(AppState::Game), despawn_hud)
            .add_systems(Update, (
                update_ammo,
                update_hourglass,
                update_health,
                update_stopwatch
            ).run_if(in_state(AppState::Game)));
    }
}