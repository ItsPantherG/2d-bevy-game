use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

    cmds.spawn(Collider::cuboid(217.5, 40.0))
        .insert(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100.0, 400.0, 8.0)),
            texture: asset_server.load("sprites/ground.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(455.0, 100.0)),
                ..default()
            },
            ..default()
        });

    cmds.spawn(Collider::cuboid(217.5, 40.0))
        .insert(SpriteBundle {
            transform: Transform::from_xyz(800.0, 200.0, 1.0),
            texture: asset_server.load("sprites/ground.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(455.0, 100.0)),
                ..default()
            },
            ..default()
        });
}
