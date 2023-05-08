use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::game::player::compoments::*;

pub const CHUNK_WIDTH: f32 = 1720.0;
const FLOOR_WIDTH: f32 = 430.0;

pub fn spawn_map(mut cmds: Commands, asset_server: Res<AssetServer>) {
    for i in 0..4 {
        cmds.spawn(Collider::cuboid(217.5, 40.0)).insert((
            SpriteBundle {
                transform: Transform::from_xyz(0.0 + (i as f32 * 430.0), 0.0, 1.0),
                texture: asset_server.load("sprites/ground.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(455.0, 100.0)),
                    ..default()
                },
                ..default()
            },
            Chunk {},
        ));
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
        let mut gap_3 = rand::thread_rng().gen_range(0..4) as f32;

        if gap_3 == gap_1 || gap_3 == gap_2 || gap_3 > 4.0 {
            gap_3 = 8.0
        }

        let normal_platform_place = rand::thread_rng().gen_range(0..4) as f32;

        //Platforms
        let height_platform_1: f32 = rand::thread_rng().gen_range(2..3) as f32 * 100.0;
        let height_platform_2: f32 = rand::thread_rng().gen_range(3..4) as f32 * 100.0;

        let random_platform_gap = rand::thread_rng().gen_range(1..230) as f32;

        for i in 0..4 {
            // Platforms Spawning
            if i as f32 == gap_1 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_1,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/normal_platform.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(227.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Chunk {},
                ));
            } else if i as f32 == gap_2 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_2,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/normal_platform.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(227.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Chunk {},
                ));
            } else if i as f32 == gap_3 && gap_3 != 8.0 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_2,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/normal_platform.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(227.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Chunk {},
                ));
            }

            // Ground Spawning
            if i as f32 != gap_1
                && i as f32 != gap_2
                && i as f32 != gap_3
                && rand::thread_rng().gen_range(0..4) > 2
            {
                cmds.spawn(Collider::cuboid(58.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            0.0,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/small_platform.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(117.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Chunk {},
                ));
            } else if i as f32 == normal_platform_place {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            0.0,
                            1.0,
                        ),
                        texture: asset_server.load("sprites/normal_platform.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(227.0, 100.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Chunk {},
                ));
            } else {
                cmds.spawn(Collider::cuboid(217.5, 40.0)).insert((
                    SpriteBundle {
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
                    },
                    Chunk {},
                ));
            }
        }
    }
}
