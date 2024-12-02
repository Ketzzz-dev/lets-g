use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::camera::*;
use crate::components::player::*;

pub fn spawn_main_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                hdr: true,

                ..default()
            },
            projection: OrthographicProjection {
                near: -1000.0,

                ..default()
            },
            tonemapping: Tonemapping::ReinhardLuminance,

            ..default()
        },
        BloomSettings::OLD_SCHOOL,
        MainCamera,
    ));
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return; };

    camera_transform.translation = player_transform.translation.clone();
}

pub fn confine_camera_bounds(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };
    let Ok(window) = window_query.get_single() else { return; };

    let half_width = 0.5 * window.width();
    let half_height = 0.5 * window.height();

    if camera_transform.translation.x - half_width < -2000.0 {
        camera_transform.translation.x = -2000.0 + half_width;
    } else if camera_transform.translation.x + half_width > 2000.0 {
        camera_transform.translation.x = 2000.0 - half_width;
    }

    if camera_transform.translation.y - half_height < -1000.0 {
        camera_transform.translation.y = -1000.0 + half_height;
    } else if camera_transform.translation.y + half_height > 1000.0 {
        camera_transform.translation.y = 1000.0 - half_height;
    }
}