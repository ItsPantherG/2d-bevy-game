use std::ptr::null;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use super::resources::EnemyShootTimer;
use crate::game::map::resources::*;
use crate::game::map::systems::CHUNK_WIDTH;
use crate::game::player::compoments::*;

pub const ENEMY_BULLET_SPEED: f32 = 300.0;

pub fn spawn_emeny_salt_thrower(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    current_chunk: Res<CurrentChunk>,
) {
    let window = window_query.get_single().unwrap();

    if current_chunk.is_changed() {
        if let Ok(player_transform) = player_query.get_single() {
            cmds.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        (current_chunk.value * CHUNK_WIDTH) + CHUNK_WIDTH,
                        100.0,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/SlimeOrange_00000.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(100.0, 80.0)),
                        ..default()
                    },
                    ..default()
                },
                EnemySaltThrower {},
            ))
            .insert(RigidBody::Fixed)
            .insert(Collider::capsule_x(20.0, 20.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(GravityScale(10.0));
        }
    }
}

pub fn enemy_shoot_player(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<EnemySaltThrower>>,
    mut enemy_shoot_timer: ResMut<EnemyShootTimer>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            if player_transform.translation.x - enemy_transform.translation.x < 300.0
                && player_transform.translation.x - enemy_transform.translation.x > -300.0
            {
                enemy_shoot_timer.timer.tick(time.delta());

                if enemy_shoot_timer.timer.just_finished() {
                    for i in 0..5 {
                        cmds.spawn((
                            SpriteBundle {
                                transform: Transform::from_xyz(
                                    enemy_transform.translation[0],
                                    enemy_transform.translation[1],
                                    10.0,
                                ),
                                texture: asset_server.load("sprites/dirt_03.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(20.0, 20.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            EnemyBullet {
                                dirction: i,
                                velocity_y: 700.0,
                            },
                        ))
                        .insert(Collider::ball(8.0))
                        .insert(RigidBody::KinematicPositionBased);
                    }
                }
            }
        }
    }
}

pub fn enemy_bullet_direction(
    mut enemy_bullet_query: Query<(&mut Transform, &mut EnemyBullet), With<EnemyBullet>>,
    enemy_shoot_timer: Res<EnemyShootTimer>,
    time: Res<Time>,
) {
    for (mut enemy_bullet_transform, mut enemy_bullet) in enemy_bullet_query.iter_mut() {
        let mut direction = Vec3::new(0.0, -500.0, 0.0);
        let decreasing_bullet_speed = enemy_shoot_timer.timer.elapsed_secs() * 3.0;

        if enemy_bullet.dirction == 0 {
            if enemy_bullet.velocity_y > 0.0 {
                enemy_bullet.velocity_y = enemy_bullet.velocity_y - decreasing_bullet_speed;
            }
            direction += Vec3::new(-300.0, enemy_bullet.velocity_y, 0.0);

            direction = direction.normalize_or_zero() * ENEMY_BULLET_SPEED * time.delta_seconds();

            enemy_bullet_transform.translation += direction
        }
        if enemy_bullet.dirction == 1 {
            if enemy_bullet.velocity_y > 0.0 {
                enemy_bullet.velocity_y = enemy_bullet.velocity_y - decreasing_bullet_speed;
            }
            direction += Vec3::new(-50.0, enemy_bullet.velocity_y, 0.0);

            direction = direction.normalize_or_zero() * ENEMY_BULLET_SPEED * time.delta_seconds();

            enemy_bullet_transform.translation += direction
        }
        if enemy_bullet.dirction == 2 {
            if enemy_bullet.velocity_y > 0.0 {
                enemy_bullet.velocity_y = enemy_bullet.velocity_y - decreasing_bullet_speed;
            }
            direction += Vec3::new(0.0, enemy_bullet.velocity_y, 0.0);

            direction = direction.normalize_or_zero() * ENEMY_BULLET_SPEED * time.delta_seconds();

            enemy_bullet_transform.translation += direction;
        }
        if enemy_bullet.dirction == 3 {
            if enemy_bullet.velocity_y > 0.0 {
                enemy_bullet.velocity_y = enemy_bullet.velocity_y - decreasing_bullet_speed;
            }
            direction += Vec3::new(50.0, enemy_bullet.velocity_y, 0.0);

            direction = direction.normalize_or_zero() * ENEMY_BULLET_SPEED * time.delta_seconds();

            enemy_bullet_transform.translation += direction
        }
        if enemy_bullet.dirction == 4 {
            if enemy_bullet.velocity_y > 0.0 {
                enemy_bullet.velocity_y = enemy_bullet.velocity_y - decreasing_bullet_speed;
            }
            direction += Vec3::new(300.0, enemy_bullet.velocity_y, 0.0);

            direction = direction.normalize_or_zero() * ENEMY_BULLET_SPEED * time.delta_seconds();

            enemy_bullet_transform.translation += direction
        }
    }
}

pub fn despawn_enemy_bullet(
    mut cmds: Commands,
    enemy_bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    for (entity, transform) in enemy_bullet_query.iter() {
        if transform.translation.y < 10.0 {
            cmds.entity(entity).despawn()
        }
    }
}
