use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::bundles::bullet::*;
use crate::bundles::dynamics::*;
use crate::components::bullet::*;
use crate::components::dynamics::*;
use crate::components::enemies::*;
use crate::components::map::*;
use crate::components::player::*;
use crate::components::weapons::*;
use crate::resources::enemies::*;

pub fn spawn_wave(
    mut commands: Commands,
    mut wave_system: ResMut<WaveSystem>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>
) {
    wave_system.timer.tick(time.delta());

    if wave_system.timer.just_finished() {
        wave_system.wave += 1;
        wave_system.max_enemies += WaveSystem::ENEMIES_PER_WAVE;

        for _ in 0..wave_system.max_enemies {
            let mut rng = thread_rng();

            let x = if rng.gen_bool(0.5) {
                Map::LEFT
            } else {
                Map::RIGHT
            };
            let y = rng.gen_range(Map::BOTTOM..Map::TOP);

            let position = Vec2::new(x, y);

            let texture = asset_server.load("textures/enemy.png");
            let layout = TextureAtlasLayout::from_grid(
                EnemyAnimationController::ATLAS_TILE_SIZE,
                EnemyAnimationController::ATLAS_COLUMNS,
                EnemyAnimationController::ATLAS_ROWS,
                None, None
            );
            let layout = texture_atlas_layout.add(layout);

            if rng.gen_bool(0.4) {
                let knife = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 4.0),
                        texture: asset_server.load("textures/knife.png"),

                        ..default()
                    },
                    Knife
                )).id();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(position.extend(3.0)),
                        texture,

                        ..default()
                    },
                    TextureAtlas::from(layout),
                    EnemyAnimationController::new(),
                    StateBundle::new(position),
                    Enemy {
                        health: Striker::MAX_HEALTH
                    },
                    Striker::new()
                )).add_child(knife);
            } else {
                let gun = commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 4.0),
                        texture: asset_server.load("textures/gun.png"),

                        ..default()
                    },
                    Gun
                )).id();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(position.extend(3.0)),
                        texture,

                        ..default()
                    },
                    TextureAtlas::from(layout),
                    EnemyAnimationController::new(),
                    StateBundle::new(position),
                    Enemy {
                        health: Shooter::MAX_HEALTH
                    },
                    Shooter::new()
                )).add_child(gun);
            }
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in &enemy_query {
        commands.entity(enemy_entity).despawn_recursive();
    }
}

pub fn move_shooters(
    mut shooter_query: Query<(&Shooter, &mut CurrentState), Without<Player>>,
    player_query: Query<&CurrentState, With<Player>>,
    time: Res<Time>
) {
    if let Ok(player_state) = player_query.get_single() {
        for (shooter, mut shooter_state) in &mut shooter_query {
            let direction = if !shooter.shooting {
                let mut direction = player_state.position - shooter_state.position;
                let distance_error = direction.length() - Shooter::DISTANCE_TO_PLAYER;

                direction *= distance_error;

                if direction.length_squared() > 0.0 {
                    direction = direction.normalize()
                }

                direction
            } else { Vec2::ZERO };

            let target_velocity = direction * Shooter::MAX_SPEED;
            let velocity_difference = target_velocity - shooter_state.velocity;

            shooter_state.velocity += velocity_difference * Shooter::ACCELERATION * time.delta_seconds();
        }
    }
}

pub fn move_strikers(
    mut striker_query: Query<(&Striker, &mut CurrentState), Without<Player>>,
    player_query: Query<&CurrentState, With<Player>>,
    time: Res<Time>
) {
    if let Ok(player_state) = player_query.get_single() {
        for (striker, mut striker_state) in &mut striker_query {
            let direction = if !striker.striking {
                let mut direction = player_state.position - striker_state.position;

                if direction.length_squared() > 0.0 {
                    direction = direction.normalize()
                }

                direction
            } else { Vec2::ZERO };

            let target_velocity = direction * Striker::MAX_SPEED;
            let velocity_difference = target_velocity - striker_state.velocity;

            striker_state.velocity += velocity_difference * Striker::ACCELERATION * time.delta_seconds();
        }
    }
}

pub fn confine_enemy_bounds(
    mut enemy_query: Query<&mut CurrentState, With<Enemy>>,
) {
    for mut enemy_state in &mut enemy_query {
        if enemy_state.position.x - Enemy::RADIUS < Map::LEFT {
            enemy_state.position.x = Map::LEFT + Enemy::RADIUS;
        } else if enemy_state.position.x + Enemy::RADIUS > Map::RIGHT {
            enemy_state.position.x = Map::RIGHT - Enemy::RADIUS;
        }

        if enemy_state.position.y - Enemy::RADIUS < Map::BOTTOM {
            enemy_state.position.y = Map::BOTTOM + Enemy::RADIUS;
        } else if enemy_state.position.y + Enemy::RADIUS > Map::TOP {
            enemy_state.position.y = Map::TOP - Enemy::RADIUS;
        }
    }
}

pub fn space_enemies(
    mut enemy_query: Query<&mut CurrentState, With<Enemy>>,
) {
    let mut combinations = enemy_query.iter_combinations_mut();

    while let Some([mut state_a, mut state_b]) = combinations.fetch_next() {
        let direction = state_b.position - state_a.position;
        let distance = direction.length();

        let enemy_diameter = 2.0 * Enemy::RADIUS;

        if distance <= enemy_diameter {
            let correction = 0.5 * direction.normalize() * (enemy_diameter - distance);

            state_a.position -= correction;
            state_b.position += correction;
        }
    }
}

pub fn shoot_shooters(
    mut commands: Commands,
    mut shooter_query: Query<(Entity, &Children, &mut Shooter, &CurrentState)>,
    gun_query: Query<(Entity, &GlobalTransform), With<Gun>>,
    player_query: Query<&CurrentState, With<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>
) {
    for (shooter_entity, shooter_children, mut shooter, shooter_state) in &mut shooter_query {
        shooter.shoot_cooldown.tick(time.delta());
        shooter.grace_period.tick(time.delta());

        if shooter.shoot_cooldown.finished() && !shooter.shooting {
            if let Ok(player_state) = player_query.get_single() {
                let direction = player_state.position - shooter_state.position;

                if direction.length() < Shooter::DISTANCE_TO_ATTACK {
                    shooter.grace_period.reset();
                    shooter.shooting = true;

                    if let Ok(direction) = Dir2::new(direction) {
                        shooter.shoot_direction = direction;
                    }
                }
            }
        }
        if shooter.grace_period.finished() && shooter.shooting {
            shooter.shoot_cooldown.reset();
            shooter.shooting = false;

            for (gun_entity, gun_transform) in &gun_query {
                for child in shooter_children {
                    if *child == gun_entity {
                        commands.spawn(BulletBundle::new(
                            shooter.shoot_direction,
                            shooter_entity,
                            gun_transform.translation().truncate(),
                            asset_server.load("textures/bullet_enemy.png"),
                            Color::srgb(5.0, 2.5, 2.5)
                        ));
                    }
                }
            }
        }
    }
}

pub fn strike_strikers(
    mut striker_query: Query<(&mut Striker, &mut CurrentState), Without<Player>>,
    player_query: Query<&CurrentState, With<Player>>,
    time: Res<Time>
) {
    for (mut striker, mut striker_state) in &mut striker_query {
        striker.strike_cooldown.tick(time.delta());
        striker.grace_period.tick(time.delta());

        if striker.strike_cooldown.finished() && !striker.striking {
            if let Ok(player_state) = player_query.get_single() {
                let direction = player_state.position - striker_state.position;

                if direction.length() < Striker::DISTANCE_TO_ATTACK {
                    striker.grace_period.reset();
                    striker.striking = true;

                    if let Ok(direction) = Dir2::new(direction) {
                        striker.strike_direction = direction;
                    }
                }
            }
        }
        if striker.grace_period.finished() && striker.striking {
            striker.strike_cooldown.reset();
            striker.striking = false;

            striker_state.velocity = 2.5 * striker.strike_direction * Striker::MAX_SPEED;
        }
    }
}

pub fn damage_enemies(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Bullet, &CurrentState)>,
    player_query: Query<Entity, With<Player>>,
    mut enemy_query: Query<(Entity, &mut Enemy, &CurrentState)>
) {
    for (bullet_entity, bullet, bullet_state) in &bullet_query {
        for (enemy_entity, mut enemy, enemy_state) in &mut enemy_query {
            if let Ok(player_entity) = player_query.get_single() {
                if bullet.owner != player_entity {
                    continue;
                }
            }

            if bullet_state.position.distance(enemy_state.position) <= Enemy::RADIUS {
                commands.entity(bullet_entity).despawn();

                enemy.health -= 50;

                if enemy.health <= 0 {
                    commands.entity(enemy_entity).despawn_recursive();
                }
            }
        }
    }
}

pub fn animate_enemies(
    mut enemy_query: Query<(&mut EnemyAnimationController, &mut TextureAtlas, &CurrentState)>,
    time: Res<Time>
) {
    for (mut animation_controller, mut enemy_atlas, enemy_state) in &mut enemy_query {
        animation_controller.timer.tick(time.delta());

        if animation_controller.timer.finished() {
            animation_controller.run_frame_index = if animation_controller.run_frame_index + 1 == EnemyAnimationController::ATLAS_COLUMNS {
                0
            } else {
                animation_controller.run_frame_index + 1
            };

            let directions = [
                enemy_state.velocity.dot(*Dir2::EAST),
                enemy_state.velocity.dot(*Dir2::NORTH_EAST),
                enemy_state.velocity.dot(*Dir2::NORTH),
                enemy_state.velocity.dot(*Dir2::NORTH_WEST),
                enemy_state.velocity.dot(*Dir2::WEST),
                enemy_state.velocity.dot(*Dir2::SOUTH_WEST),
                enemy_state.velocity.dot(*Dir2::SOUTH),
                enemy_state.velocity.dot(*Dir2::SOUTH_EAST)
            ];

            let mut max_direction = directions[0];
            let mut direction_index = EnemyAnimationController::DIRECTION_INDICES[0];

            for i in 1..8 {
                if directions[i] > max_direction {
                    max_direction = directions[i];
                    direction_index = EnemyAnimationController::DIRECTION_INDICES[i];
                }
            }

            if enemy_state.velocity.length_squared() > 750.0 {  
                enemy_atlas.index = (EnemyAnimationController::ATLAS_COLUMNS + (EnemyAnimationController::ATLAS_COLUMNS * direction_index) + animation_controller.run_frame_index) as usize;
            } else {
                enemy_atlas.index = direction_index as usize;
            }
        }
    }
}

pub fn point_shooter_gun_to_player(
    shooter_query: Query<(&Children, &CurrentState), With<Shooter>>,
    mut gun_query: Query<(Entity, &mut Transform), With<Gun>>,
    player_query: Query<&CurrentState, With<Player>>
) {
    for (shooter_children, shooter_state) in &shooter_query {
        for (gun_entity, mut gun_transform) in &mut gun_query {
            for child in shooter_children {
                if *child == gun_entity {
                    let Ok(player_state) = player_query.get_single() else { return; };

                    let direction = player_state.position - shooter_state.position;

                    let translation = direction.normalize() * Enemy::RADIUS;

                    gun_transform.translation = Vec3::new(translation.x, 0.5 * Enemy::RADIUS + translation.y, gun_transform.translation.z);
                    gun_transform.rotation = Quat::from_rotation_z(direction.to_angle());
                }
            }
        }
    }
}

pub fn point_striker_knife_to_player(
    shooter_query: Query<(&Children, &CurrentState), With<Striker>>,
    mut gun_query: Query<(Entity, &mut Transform), With<Knife>>,
    player_query: Query<&CurrentState, With<Player>>
) {
    for (striker_children, striker_state) in &shooter_query {
        for (knife_entity, mut knife_transform) in &mut gun_query {
            for child in striker_children {
                if *child == knife_entity {
                    let Ok(player_state) = player_query.get_single() else { return; };

                    let direction = player_state.position - striker_state.position;

                    let translation = direction.normalize() * Enemy::RADIUS;

                    knife_transform.translation = Vec3::new(translation.x, 0.5 * Enemy::RADIUS + translation.y, knife_transform.translation.z);
                    knife_transform.rotation = Quat::from_rotation_z(direction.to_angle());
                }
            }
        }
    }
}