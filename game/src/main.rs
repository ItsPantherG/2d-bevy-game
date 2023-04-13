pub mod actions;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::actions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(player_move)
        .run()
}

pub const PLAYER_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Player {}

pub fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    cmds.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Camera,
    ));

    for i in 0..3 {
        cmds.spawn(Collider::cuboid(227.5, 40.0))
            .insert(SpriteBundle {
                transform: Transform::from_xyz(0.0 + (i as f32 * 430.0), 0.0, 0.0),
                texture: asset_server.load("sprites/ground.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(455.0, 100.0)),
                    ..default()
                },
                ..default()
            });
    }

    cmds.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/SlimeBasic_00000.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 68.0)),
                ..default()
            },
            ..default()
        },
        Player {},
    ))
    .insert(KinematicCharacterController {
        snap_to_ground: Some(CharacterLength::Absolute(0.5)),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(30.0))
    .insert(Restitution::coefficient(0.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(GravityScale(5.0));
}

pub fn player_move(
    mut controllers: Query<(&mut KinematicCharacterController)>,
    time: Res<Time>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(Action::Left) {
            direction += Vec2::new(-1.0, 0.0)
        }
        if keyboard_input.pressed(Action::Right) {
            direction += Vec2::new(1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize() * PLAYER_SPEED * time.delta_seconds()
        }

        controller.translation = Some(direction);
    }
}
