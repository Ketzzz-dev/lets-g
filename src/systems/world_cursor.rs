use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::camera::*;
use crate::resources::world_cursor::*;

pub fn update_world_cursor(
    mut world_cursor: ResMut<WorldCursor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    if let Ok(window) = window_query.get_single() {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Some(world_position) = window.cursor_position().and_then(|position| camera.viewport_to_world_2d(camera_transform, position)) {
                world_cursor.position = world_position;
            }
        }
    }
}