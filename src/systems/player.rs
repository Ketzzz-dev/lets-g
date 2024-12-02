use bevy::prelude::*;
use crate::bundles::bullet::*;
use crate::bundles::dynamics::*;
use crate::components::bullet::*;
use crate::components::dynamics::*;
use crate::components::enemies::{Enemy, Striker};
use crate::components::map::*;
use crate::components::player::*;
use crate::components::weapons::*;
use crate::events::game_over::*;
use crate::resources::timestop::*;
use crate::resources::world_cursor::*;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>
) {
    let texture = asset_server.load("textures/player.png");
    let layout = TextureAtlasLayout::from_grid(
        PlayerAnimationController::ATLAS_TILE_SIZE,
        PlayerAnimationController::ATLAS_COLUMNS,
        PlayerAnimationController::ATLAS_ROWS,
        None, None
    );
    let layout = texture_atlas_layout.add(layout);

    let gun = commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            texture: asset_server.load("textures/gun.png"),

            ..default()
        },
        Gun
    )).id();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture,

            ..default()
        },
        TextureAtlas::from(layout),
        PlayerAnimationController::new(),
        StateBundle::default(),
        Player::new(),
    )).add_child(gun);
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}

pub fn move_player(
    mut player_query: Query<&mut CurrentState, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let Ok(mut player_state) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec2::NEG_X;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec2::X;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec2::Y;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec2::NEG_Y;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let target_velocity = direction * Player::MAX_SPEED;
    let velocity_difference = target_velocity - player_state.velocity;

    let factor = if target_velocity.length_squared() > 0.0 {
        Player::ACCELERATION
    } else {
        Player::DECELERATION
    };

    player_state.velocity += velocity_difference * factor * time.delta_seconds();
}

pub fn confine_player_bounds(
    mut player_query: Query<&mut CurrentState, With<Player>>,
) {
    let Ok(mut player_state) = player_query.get_single_mut() else { return };

    if player_state.position.x - Player::RADIUS < Map::LEFT {
        player_state.position.x = Map::LEFT + Player::RADIUS;
    } else if player_state.position.x + Player::RADIUS > Map::RIGHT {
        player_state.position.x = Map::RIGHT - Player::RADIUS;
    }

    if player_state.position.y - Player::RADIUS < Map::BOTTOM {
        player_state.position.y = Map::BOTTOM + Player::RADIUS;
    } else if player_state.position.y + Player::RADIUS > Map::TOP {
        player_state.position.y = Map::TOP - Player::RADIUS;
    }
}

pub fn shoot_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Children, &mut Player, &CurrentState)>,
    gun_query: Query<(Entity, &GlobalTransform), With<Gun>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    world_cursor: Res<WorldCursor>,
    asset_server: Res<AssetServer>,
    time: Res<Time>
) {
    let Ok((
        player_entity,
        player_children,
        mut player,
        player_state
    )) = player_query.get_single_mut() else { return; };

    player.shoot_cooldown.tick(time.delta());

    if mouse_input.just_pressed(MouseButton::Left) && player.shoot_cooldown.finished() && player.reload_timer.finished() && player.ammo > 0 {
        player.shoot_cooldown.reset();
        player.ammo -= 1;

        let Ok(direction) = Dir2::new(world_cursor.position - player_state.position) else {
            return;
        };

        for (gun_entity, gun_transform) in &gun_query {
            for child in player_children {
                if *child == gun_entity {
                    commands.spawn(BulletBundle::new(
                        direction,
                        player_entity,
                        gun_transform.translation().truncate(),
                        asset_server.load("textures/bullet_player.png"),
                        Color::srgb(2.5, 2.5, 5.0)
                    ));
                }
            }
        }
    }
}

pub fn stop_time(
    mut timestop: ResMut<Timestop>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    timestop.stop_timer.tick(time.delta());
    timestop.cooldown_timer.tick(time.delta());

    if keyboard_input.just_pressed(KeyCode::Space) && timestop.cooldown_timer.finished() {
        timestop.stop_timer.reset();

        timestop.active = true;
    }
    if timestop.stop_timer.just_finished() {
        timestop.cooldown_timer.reset();

        timestop.active = false;
    }
}

pub fn reload_player(
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };

    player.reload_timer.tick(time.delta());

    if player.reload_timer.just_finished() {
        player.ammo = Player::MAX_AMMO;
    }
    if keyboard_input.just_pressed(KeyCode::KeyR) && player.reload_timer.finished() && player.ammo < 6 {
        player.reload_timer.reset();
    }
}

pub fn animate_player(
    mut player_query: Query<(&mut PlayerAnimationController, &mut TextureAtlas, &CurrentState)>,
    time: Res<Time>
) {
    let Ok((
        mut animation_controller,
        mut player_atlas,
        player_state
    )) = player_query.get_single_mut() else { return; };

    animation_controller.timer.tick(time.delta());

    if animation_controller.timer.finished() {
        animation_controller.run_frame_index = if animation_controller.run_frame_index + 1 == PlayerAnimationController::ATLAS_COLUMNS {
            0
        } else {
            animation_controller.run_frame_index + 1
        };

        let directions = [
            player_state.velocity.dot(*Dir2::EAST),
            player_state.velocity.dot(*Dir2::NORTH_EAST),
            player_state.velocity.dot(*Dir2::NORTH),
            player_state.velocity.dot(*Dir2::NORTH_WEST),
            player_state.velocity.dot(*Dir2::WEST),
            player_state.velocity.dot(*Dir2::SOUTH_WEST),
            player_state.velocity.dot(*Dir2::SOUTH),
            player_state.velocity.dot(*Dir2::SOUTH_EAST)
        ];

        let mut max_direction = directions[0];
        let mut direction_index = PlayerAnimationController::DIRECTION_INDICES[0];

        for i in 1..8 {
            if directions[i] > max_direction {
                max_direction = directions[i];
                direction_index = PlayerAnimationController::DIRECTION_INDICES[i];
            }
        }

        if player_state.velocity.length_squared() > 750.0 {
            player_atlas.index = (PlayerAnimationController::ATLAS_COLUMNS + (PlayerAnimationController::ATLAS_COLUMNS * direction_index) + animation_controller.run_frame_index) as usize;
        } else {
            player_atlas.index = direction_index as usize;
        }
    }
}

pub fn point_player_gun_to_cursor(
    player_query: Query<(&Children, &CurrentState), With<Player>>,
    mut gun_query: Query<(Entity, &mut Transform), With<Gun>>,
    world_cursor: Res<WorldCursor>
) {
    let Ok((player_children, player_state)) = player_query.get_single() else { return; };

    for (gun_entity, mut gun_transform) in &mut gun_query {
        for child in player_children {
            if *child == gun_entity {
                let direction = world_cursor.position - player_state.position;

                let translation = direction.normalize() * Player::RADIUS;

                gun_transform.translation = Vec3::new(translation.x, 0.5 * Player::RADIUS + translation.y, gun_transform.translation.z);
                gun_transform.rotation = Quat::from_rotation_z(direction.to_angle());
            }
        }
    }
}

pub fn damage_player(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Bullet, &CurrentState)>,
    striker_query: Query<&CurrentState, (With<Striker>, Without<Player>, Without<Bullet>)>,
    mut player_query: Query<(Entity, &mut Player, &CurrentState)>,
    mut game_over_event: EventWriter<GameOverEvent>
) {
    let Ok((player_entity, mut player, player_state)) = player_query.get_single_mut() else { return; };

    for (bullet_entity, bullet, bullet_state) in &bullet_query {
        if bullet.owner == player_entity {
            continue;
        }

        if bullet_state.position.distance(player_state.position) <= Player::RADIUS {
            commands.entity(bullet_entity).despawn();

            player.health -= 10.0;

            if player.health <= 0.0 {
                commands.entity(player_entity).despawn_recursive();

                game_over_event.send(GameOverEvent);
            }
        }
    }
    for striker_state in &striker_query {
        if striker_state.position.distance(player_state.position) <= Player::RADIUS + Enemy::RADIUS {
            player.health -= 10.0;

            if player.health <= 0.0 {
                commands.entity(player_entity).despawn_recursive();

                game_over_event.send(GameOverEvent);
            }
        }
    }
}

pub fn regenerate_player(
    mut player_query: Query<&mut Player>,
    time: Res<Time>
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        player.health += time.delta_seconds();

        player.health = player.health.clamp(0.0, Player::MAX_HEALTH);
    }
}

pub fn time_paused(timestop: Res<Timestop>) -> bool {
    !timestop.active
}