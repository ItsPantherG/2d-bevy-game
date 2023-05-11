use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
                transform: Transform::from_xyz(0.0 + (i as f32 * 430.0), 0.0, 101.0),
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

pub fn despawn_map(
    mut cmds: Commands,
    chunk_query: Query<Entity, With<Chunk>>,
    background_query: Query<Entity, With<Background>>,
) {
    for chunk_entity in chunk_query.iter() {
        cmds.entity(chunk_entity).despawn();
    }

    for background_entity in background_query.iter() {
        cmds.entity(background_entity).despawn();
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
    if current_chunk.is_changed() && current_chunk.value >= 0.0 {
        //random nnumbers
        let random_2 = rand::thread_rng().gen_range(0..2);

        /*
        ======== CHUNK SPAWNING ========
        - A chunk is 1720 pixels in width and will be spawned as soon as the player enters a new chunk. The spawning shunk will be one chunk ahead of the player.
        So if the player enters in chunk: 1, chunk 2 will be spawned, if the player enters chunk 2, chunk 3 will be spawned

        ======== CHUNK GENERATION ========

        - Chunks will be generated be a random loop of 0..4
        - Every chunk will have 2 or 3 gaps, those gaps will have a random generated number between 0 and 4 (0, 1, 2, 3). According to those number a platform will
            spawn. So if the gaps are "0" and "2", a platform will spawn on the first and third iteration.
        - The gaps also serve as an indicator for a small floor spawn: On the iteration where there are no gaps (2 if there are 2 gaps, and 3 if there are
            3 gaps) will be a 25% chance of spawning a small platform to jump over.

        - There will also ALWAYS spawn a normal sized platform the will create a small jump or a large jump if combined with a small platform.

        - The rest of the iterations a large platform will spawn that leaves no gap in between

        */

        //Ground
        //amount of platforms per chunk, 2 or 3
        let gap_1 = rand::thread_rng().gen_range(0..2) as f32;
        let gap_2 = rand::thread_rng().gen_range(2..4) as f32;
        let mut gap_3 = rand::thread_rng().gen_range(0..4) as f32;
        if gap_3 == gap_1 || gap_3 == gap_2 || gap_3 > 4.0 {
            gap_3 = 8.0
        }

        // normal size platform place, always ONE per chunk
        let normal_platform_place = rand::thread_rng().gen_range(0..4) as f32;

        //Platforms
        let height_platform_1: f32 = rand::thread_rng().gen_range(2..3) as f32 * 100.0;
        let height_platform_2: f32 = rand::thread_rng().gen_range(3..4) as f32 * 100.0;

        let random_platform_gap = rand::thread_rng().gen_range(1..230) as f32;

        for i in 0..4 {
            // Platforms Spawning======================================================================
            if i as f32 == gap_1 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_1,
                            101.0,
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

                if rand::thread_rng().gen_range(0..2) == 1 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + random_platform_gap
                                        + rand::thread_rng().gen_range(-40.0..40.0)),
                                height_platform_1 + 90.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/grass_1.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(100.0, 98.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                }
            } else if i as f32 == gap_2 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_2,
                            101.0,
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

                if rand::thread_rng().gen_range(0..2) == 1 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + random_platform_gap
                                        + rand::thread_rng().gen_range(-40.0..40.0)),
                                height_platform_2 + 90.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/grass_1.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(100.0, 98.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                }
            } else if i as f32 == gap_3 && gap_3 != 8.0 {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                + (i as f32 * FLOOR_WIDTH + random_platform_gap),
                            height_platform_2,
                            101.0,
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

            // Ground Spawning======================================================================
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
                            101.0,
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

                if rand::thread_rng().gen_range(0..2) > 1 {
                    cmds.spawn(SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            90.0,
                            0.0,
                        ),
                        texture: asset_server.load("sprites/grass_3.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100.0, 98.0)),
                            ..default()
                        },
                        ..default()
                    });
                }
            } else if i as f32 == normal_platform_place {
                cmds.spawn(Collider::cuboid(113.5, 40.0)).insert((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            (CHUNK_WIDTH * (current_chunk.value + 1.0)) + (i as f32 * FLOOR_WIDTH),
                            0.0,
                            101.0,
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
                            101.0,
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

                if rand::thread_rng().gen_range(0..5) == 1 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + rand::thread_rng().gen_range(-100.0..100.0)),
                                100.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/rock_1.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(250.0, 127.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                } else if rand::thread_rng().gen_range(0..6) == 0 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + rand::thread_rng().gen_range(-100.0..100.0)),
                                150.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/rock_3.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(300.0, 257.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                }

                if rand::thread_rng().gen_range(0..2) == 1 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + random_platform_gap
                                        + rand::thread_rng().gen_range(-60.0..30.0)),
                                90.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/grass_1.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(100.0, 98.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                }
                if rand::thread_rng().gen_range(0..2) == 1 {
                    cmds.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (CHUNK_WIDTH * (current_chunk.value + 1.0))
                                    + (i as f32 * FLOOR_WIDTH
                                        + random_platform_gap
                                        + rand::thread_rng().gen_range(-60.0..30.0)),
                                90.0,
                                0.0,
                            ),
                            texture: asset_server.load("sprites/grass_3.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(100.0, 98.0)),
                                ..default()
                            },
                            ..default()
                        },
                        Background {},
                    ));
                }
            }
        }
    }
}

pub fn generate_background_image(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    cmds.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0 - 150.0, 0.0),
        texture: asset_server.load("sprites/background_image_1.png"),
        ..default()
    });
}
