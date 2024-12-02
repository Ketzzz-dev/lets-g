mod components;
mod systems;
mod resources;
mod bundles;
mod plugins;
mod events;

use bevy::prelude::*;
use bevy::window::*;
use crate::plugins::game::*;
use crate::plugins::game_over::*;
use crate::plugins::menu::*;
use crate::resources::world_cursor::*;
use crate::systems::camera::*;
use crate::systems::world_cursor::*;

#[derive(States, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    Menu,
    Game,
    GameOver
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "STOP;WATCH".into(),
                    name: Some("STOP;WATCH".into()),
                    present_mode: PresentMode::AutoNoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    mode: WindowMode::Fullscreen,

                    ..default()
                }),

                ..default()
            }),
            MenuPlugin,
            GamePlugin,
            GameOverPlugin
        ))
        .init_state::<AppState>()
        .init_resource::<WorldCursor>()
        .add_systems(Startup, spawn_main_camera)
        .add_systems(Update, update_world_cursor)
        .run();
}