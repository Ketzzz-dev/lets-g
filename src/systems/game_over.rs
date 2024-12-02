use bevy::prelude::*;
use crate::AppState;
use crate::components::game_over::*;
use crate::events::game_over::*;

pub fn check_game_over(
    mut game_over_event: EventReader<GameOverEvent>,
    mut next_state: ResMut<NextState<AppState>>
) {
    for _ in game_over_event.read() {
        next_state.set(AppState::GameOver);
    }
}

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        ImageBundle {
            image: UiImage::new(asset_server.load("textures/game_over.png")),

            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..default()
            },

            ..default()
        },
        GameOver::new()
    ));
}

pub fn despawn_game_over(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOver>>
) {
    if let Ok(game_over_entity) = game_over_query.get_single() {
        commands.entity(game_over_entity).despawn();
    }
}

pub fn tick_game_over(
    mut game_over_query: Query<&mut GameOver>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>
) {
    let Ok(mut game_over) = game_over_query.get_single_mut() else { return; };

    game_over.timer.tick(time.delta());

    if game_over.timer.finished() {
        next_state.set(AppState::Menu);
    }
}