use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::resources::*;
use crate::game::player::compoments::*;

const CHUNK_WIDTH: f32 = 1720.0;
const FLOOR_WIDTH: f32 = 430.0;

pub fn spawn_map(mut cmds: Commands, asset_server: Res<AssetServer>) {
    for i in 0..4 {
        cmds.spawn(Collider::cuboid(217.5, 40.0))
            .insert(SpriteBundle {
                transform: Transform::from_xyz(0.0 + (i as f32 * 430.0), 0.0, 1.0),
                texture: asset_server.load("sprites/ground.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(455.0, 100.0)),
                    ..default()
                },
                ..default()
            });
    }
}

pub fn change_chunks(
    player_query: Query<&Transform, With<Player>>,
    mut current_chunk: ResMut<CurrentChunk>,
) {
    let new_chunk: f32;
    if let Ok(transform_player) = player_query.get_single() {
        new_chunk = (transform_player.translation.x / CHUNK_WIDTH).floor();

        if new_chunk > current_chunk.value {
            current_chunk.value = new_chunk
        }
    }
}

pub fn generate_random_chunk(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    current_chunk: Res<CurrentChunk>,
) {
    if current_chunk.is_changed() {
        //Ground
        let gap_1 = rand::thread_rng().gen_range(0..2) as f32;
        let gap_2 = rand::thread_rng().gen_range(2..4) as f32;

        //Platforms
        let height_platform_1: f32 = 2.0 * 100.0;
        let height_platform_2: f32 = 4.0 * 100.0;

        for i in 0..4 {
            if i as f32 == gap_1 {
                cmds.spawn(Collider::cuboid(217.5, 40.0))
                    .insert(SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            height_platform_1,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/ground.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(455.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    });
            } else if i as f32 == gap_2 {
                cmds.spawn(Collider::cuboid(217.5, 40.0))
                    .insert(SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            height_platform_2,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/ground.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(455.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    });
            } else {
                cmds.spawn(Collider::cuboid(217.5, 40.0))
                    .insert(SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            0.0,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/ground.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(455.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    });
            }
        }
    }
}
