use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::super::compoments::*;
use super::components::*;

pub const BULLET_SPEED: f32 = 1000.0;

pub fn spawn_bullet(
    mut cmds: Commands,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mouse_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(_position) = window.cursor_position() {
            if let Ok(player_transform) = player_query.get_single() {
                cmds.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            player_transform.translation[0],
                            player_transform.translation[1],
                            0.0,
                        ),
                        texture: asset_server.load("sprites/tile_0000.png"),
                        ..default()
                    },
                    Bullet {
                        direction: _position - Vec2::new(640.0, player_transform.translation[1]),
                        ..default()
                    },
                ))
                .insert(KinematicCharacterController {
                    filter_groups: Some(CollisionGroups {
                        memberships: Group::GROUP_4,
                        filters: Group::GROUP_2,
                    }),
                    ..default()
                })
                .insert(Collider::ball(10.0))
                .insert(RigidBody::Dynamic)
                .insert(CollisionGroups {
                    memberships: Group::GROUP_4,
                    filters: Group::GROUP_2,
                });
            }
        }
    }
}

pub fn bullet_direction(
    mut bullet_query: Query<(&mut KinematicCharacterController, &Bullet), With<Bullet>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    for (mut transform, bullet) in bullet_query.iter_mut() {
        direction += Vec2::new(bullet.direction[0], bullet.direction[1]);

        direction = direction.normalize_or_zero() * BULLET_SPEED * time.delta_seconds();

        transform.translation = Some(direction)
    }
}

pub fn despawn_bullet_on_collision(
    mut cmds: Commands,
    bullets_output: Query<(Entity, &KinematicCharacterControllerOutput), With<Bullet>>,
) {
    for (entity, output) in bullets_output.iter() {
        if !output.collisions.is_empty() {
            cmds.entity(entity).despawn()
        }
    }
}

pub fn despawn_bullet_over_time(
    mut cmds: Commands,
    mut bullet_query: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullet_query.iter_mut() {
        bullet.timer.tick(time.delta());

        if bullet.timer.just_finished() {
            cmds.entity(entity).despawn();
        }
    }
}
