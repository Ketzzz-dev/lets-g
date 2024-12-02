use bevy::prelude::*;
use crate::AppState;
use crate::systems::menu::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnExit(AppState::Menu), despawn_menu)
            .add_systems(Update, handle_options.run_if(in_state(AppState::Menu)));
    }
}