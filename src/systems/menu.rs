use bevy::prelude::*;
use crate::AppState;
use crate::components::menu::*;

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            ImageBundle {
                image: UiImage::new(asset_server.load("textures/background.png")),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    flex_direction: FlexDirection::Column,

                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::Center,

                    ..default()
                },

                ..default()
            },
            Menu
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(asset_server.load("textures/title.png")),

                    ..default()
                },
                Title
            ));
            parent.spawn((
                TextBundle::from_section(
                    "ESC to Quit\nEnter to Play",
                    TextStyle {
                        font: asset_server.load("fonts/giantoli_serif.ttf"),
                        font_size: 64.0,

                        ..default()
                    }
                ),
                Options
            ));
        });
}

pub fn despawn_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>
) {
    let Ok(menu_entity) = menu_query.get_single() else { return; };

    commands.entity(menu_entity).despawn_recursive();
}

pub fn handle_options(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_app_state.set(AppState::Game);
    }
}