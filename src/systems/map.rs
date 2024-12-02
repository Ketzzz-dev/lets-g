use bevy::prelude::*;
use crate::components::map::*;

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            texture: asset_server.load("textures/map.png"),

            ..default()
        },
        Map
    ));
}

pub fn despawn_map(
    mut commands: Commands,
    map_query: Query<Entity, With<Map>>
) {
    if let Ok(map_entity) = map_query.get_single() {
        commands.entity(map_entity).despawn();
    }
}