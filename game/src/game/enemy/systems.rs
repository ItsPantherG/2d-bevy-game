use bevy::prelude::*;
use bevy::transform;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::game::map::resources::*;
use crate::game::map::systems::CHUNK_WIDTH;
use crate::game::player::compoments::*;

pub const ENEMY_BULLET_SPEED: f32 = 300.0;

pub fn spawn_emeny_salt_thrower(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    current_chunk: Res<CurrentChunk>,
) {
    if current_chunk.is_changed() {
        if rand::thread_rng().gen_range(0..2) == 1 {
            cmds.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        (current_chunk.value * CHUNK_WIDTH) + CHUNK_WIDTH,
                        60.0,
                        10.0,
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
    enemy_bullet_query: Query<(Entity, &KinematicCharacterControllerOutput), With<EnemyBullet>>,
    mut enemy_shoot_timer: ResMut<EnemyShootTimer>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            if player_transform.translation.x - enemy_transform.translation.x < 400.0
                && player_transform.translation.x - enemy_transform.translation.x > -400.0
            {
                enemy_shoot_timer.timer.tick(time.delta());

                if enemy_shoot_timer.timer.finished() && enemy_bullet_query.is_empty() {
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
                            desired_direction: player_transform.translation.x
                                - enemy_transform.translation.x,
                        },
                        EnemyBulletTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                    ))
                    .insert(KinematicCharacterController::default())
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(Collider::ball(8.0));
                }
            }
        }
    }
}

pub fn start_enemy_bullet_timer(
    mut enemy_bullet_timer_query: Query<
        &mut EnemyBulletTimer,
        (With<EnemyBulletTimer>, Without<EnemyBulletDespawnTimer>),
    >,
    mut enemy_bullet_despawn_timer_query: Query<
        &mut EnemyBulletTimer,
        (With<EnemyBulletDespawnTimer>, Without<EnemyBulletTimer>),
    >,
    time: Res<Time>,
) {
    for mut timer in enemy_bullet_timer_query.iter_mut() {
        timer.tick(time.delta());
    }
    for mut despawn_timer in enemy_bullet_despawn_timer_query.iter_mut() {
        despawn_timer.tick(time.delta());
    }
}

pub fn enemy_bullet_direction(
    mut enemy_bullet_query: Query<
        (&mut KinematicCharacterController, &EnemyBullet),
        With<EnemyBullet>,
    >,
    enemy_bullet_timer_query: Query<&EnemyBulletTimer, With<EnemyBulletTimer>>,
    time: Res<Time>,
) {
    if let Ok((mut enemy_bullet_transform, enemy_bullet)) = enemy_bullet_query.get_single_mut() {
        for bullet_timer in enemy_bullet_timer_query.iter() {
            let mut direction = Vec2::ZERO;

            let delta_enemy_player = enemy_bullet.desired_direction;
            let desired_direction = if delta_enemy_player > 0.0 { 1.0 } else { -1.0 };
            let bullet_range = if delta_enemy_player > 0.0 {
                (delta_enemy_player - 50.0) / 100.0
            } else {
                -(delta_enemy_player + 50.0) / 100.0
            };

            direction += Vec2::new(desired_direction, 0.0);

            if direction.length() > 0.0 {
                direction = direction.normalize() * (ENEMY_BULLET_SPEED * bullet_range);
            }

            direction += Vec2::new(0.0, bullet_timer.remaining_secs()) * 3500.0;
            direction += Vec2::new(0.0, 1.0 * -1000.0);
            direction = direction * time.delta_seconds();

            enemy_bullet_transform.translation = Some(direction)
        }
    }
}

pub fn despawn_enemy_bullet_on_collision(
    mut cmds: Commands,
    enemy_bullet_query: Query<
        (Entity, &KinematicCharacterControllerOutput, &Transform),
        (With<EnemyBullet>, Without<Player>),
    >,
    player_query: Query<Entity, (With<Player>, Without<EnemyBullet>)>,
) {
    if let Ok((entity, output, transform)) = enemy_bullet_query.get_single() {
        if let Ok(player_entity) = player_query.get_single() {
            if !output.collisions.is_empty() {
                for collision in output.collisions.iter() {
                    if collision.entity == player_entity {
                        println!("hit")
                    }
                }
                cmds.entity(entity).despawn()
            }
            let translation = transform.translation;

            if translation.y < -40.0 {
                cmds.entity(entity).despawn()
            }
        }
    }
}
