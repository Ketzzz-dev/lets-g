
use bevy::prelude::*;
use crate::components::hud::*;
use crate::components::player::*;
use crate::resources::enemies::*;
use crate::resources::timestop::*;

pub fn spawn_hud(
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    flex_direction: FlexDirection::Column,

                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::SpaceBetween,

                    padding: UiRect::all(Val::Px(8.0)),

                    ..default()
                },

                ..default()
            },
            Root
        ))
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),

                        flex_direction: FlexDirection::Column,

                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,

                        ..default()
                    },

                    ..default()
                }
            ).with_children(|parent| {
                let texture = asset_server.load("textures/hourglass.png");
                let layout = TextureAtlasLayout::from_grid(
                    Hourglass::ATLAS_TILE_SIZE,
                    Hourglass::ATLAS_COLUMNS,
                    Hourglass::ATLAS_ROWS,
                    None, None
                );
                let layout = texture_atlas_layout.add(layout);

                parent.spawn((
                    ImageBundle {
                        image: UiImage::new(texture),
                        style: Style {
                            width: Val::Px(128.0),
                            height: Val::Px(128.0),

                            ..default()
                        },

                        ..default()
                    },
                    TextureAtlas::from(layout),
                    Hourglass
                ));
            });

            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),

                        flex_direction: FlexDirection::Row,

                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceBetween,

                        ..default()
                    },

                    ..default()
                }
            ).with_children(|parent| {
                let texture = asset_server.load("textures/ammo.png");
                let layout = TextureAtlasLayout::from_grid(
                    Ammo::ATLAS_TILE_SIZE,
                    Ammo::ATLAS_COLUMNS,
                    Ammo::ATLAS_ROWS,
                    None, None
                );
                let layout = texture_atlas_layout.add(layout);

                parent.spawn((
                    ImageBundle {
                        image: UiImage::new(texture),
                        style: Style {
                            width: Val::Px(256.0),
                            height: Val::Px(256.0),

                            ..default()
                        },

                        ..default()
                    },
                    TextureAtlas::from(layout),
                    Ammo
                ));
                parent.spawn((
                    TextBundle::from_section(
                        "100",
                        TextStyle {
                            font: asset_server.load("fonts/giantoli_serif.ttf"),
                            font_size: 64.0,

                            ..default()
                        }
                    ),
                    Health
                ));
                parent.spawn((
                    ImageBundle {
                        image: UiImage::new(asset_server.load("textures/stopwatch.png")),
                        style: Style {
                            width: Val::Px(256.0),
                            height: Val::Px(256.0),

                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,

                            ..default()
                        },

                        ..default()
                    },
                    Stopwatch
                )).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/giantoli_serif.ttf"),
                                font_size: 64.0,

                                ..default()
                            }
                        )
                    );
                });
            });
        });
}

pub fn update_ammo(
    mut ammo_query: Query<&mut TextureAtlas, With<Ammo>>,
    player_query: Query<&Player>
) {
    let Ok(mut ammo_texture_atlas) = ammo_query.get_single_mut() else { return; };
    let Ok(player) = player_query.get_single() else { return; };

    ammo_texture_atlas.index = Ammo::AMMO_INDICES[player.ammo as usize];
}

pub fn update_hourglass(
    mut hourglass_query: Query<&mut TextureAtlas, With<Hourglass>>,
    wave_system: Res<WaveSystem>
) {
    let Ok(mut hourglass_texture_atlas) = hourglass_query.get_single_mut() else { return; };

    let index = wave_system.timer.fraction() * Hourglass::FRAMES as f32;

    hourglass_texture_atlas.index = index as usize;
}

pub fn update_stopwatch(
    mut stopwatch_query: Query<(&mut UiImage, &Children), With<Stopwatch>>,
    mut text_query: Query<(Entity, &mut Text)>,
    timestop: Res<Timestop>
) {
    let Ok((mut stopwatch_image, stopwatch_children)) = stopwatch_query.get_single_mut() else { return; };

    if timestop.active {
        stopwatch_image.color = Color::srgb(1.5, 1.5, 1.5);
    } else if timestop.cooldown_timer.remaining_secs() > 0.0 {
        stopwatch_image.color = Color::srgb(0.5, 0.5, 0.5);

        for child in stopwatch_children {
            for (text_entity, mut text) in &mut text_query {
                if *child == text_entity {
                    text.sections[0].value = timestop.cooldown_timer.remaining().as_secs().to_string();
                }
            }
        }
    } else {
        stopwatch_image.color = Color::srgb(1.0, 1.0, 1.0);

        for child in stopwatch_children {
            for (text_entity, mut text) in &mut text_query {
                if *child == text_entity {
                    text.sections[0].value = "".into();
                }
            }
        }
    }
}

pub fn update_health(
    mut health_query: Query<&mut Text, With<Health>>,
    player_query: Query<&Player>
) {
    let Ok(mut health_text) = health_query.get_single_mut() else { return; };
    let Ok(player) = player_query.get_single() else { return; };

    health_text.sections[0].value = (player.health as u32).to_string();
}

pub fn despawn_hud(
    mut commands: Commands,
    root_node_query: Query<Entity, With<Root>>
) {
    if let Ok(root_node_entity) = root_node_query.get_single() {
        commands.entity(root_node_entity).despawn_recursive();
    }
}