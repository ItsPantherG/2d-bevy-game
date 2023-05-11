use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::compoments::*;
use super::resources::*;
use super::MovementState;
use crate::game::enemy::components::*;
use crate::GameState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const JUMP_STRENGTH: f32 = 5000.0;
pub const AIR_DASH_SPEED: f32 = 13000.0;
pub const GRAVITY_STRENGTH: f32 = -1000.0;

pub fn spawn_player(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    cmds.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
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
        up: Vec2::Y,
        snap_to_ground: Some(CharacterLength::Relative(0.2)),
        min_slope_slide_angle: (50.0_f32).to_radians(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(CollisionGroups {
        memberships: Group::GROUP_4,
        filters: Group::GROUP_2,
    })
    .insert(Collider::capsule_x(20.0, 20.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(GravityScale(5.0))
    .insert(Name::new("Player"));
}

pub fn player_move(
    mut controllers: Query<&mut KinematicCharacterController, With<Player>>,
    controllers_output: Query<&KinematicCharacterControllerOutput, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    current_state: Res<State<MovementState>>,
    jump_timer: Res<JumpTimer>,
    air_dash: Res<AirDash>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec2::ZERO;

        // Left Right
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec2::new(1.0, 0.0);
        }

        // Normalize and set player speed
        if direction.length() > 0.0 {
            direction = direction.normalize_or_zero() * PLAYER_SPEED;
        }

        // set jump speed
        if current_state.0 == MovementState::Jumping {
            direction += Vec2::new(0.0, jump_timer.timer.remaining_secs()) * JUMP_STRENGTH;
        }

        // air dash
        if current_state.0 == MovementState::AirDash {
            for output in controllers_output.iter() {
                let player_movement = output.effective_translation[0];
                if player_movement > 0.0 {
                    direction += Vec2::new(
                        air_dash.timer.elapsed_secs(),
                        -GRAVITY_STRENGTH / AIR_DASH_SPEED,
                    ) * AIR_DASH_SPEED;
                }
                if player_movement < 0.0 {
                    direction += Vec2::new(
                        -air_dash.timer.elapsed_secs(),
                        -GRAVITY_STRENGTH / AIR_DASH_SPEED,
                    ) * AIR_DASH_SPEED;
                }
            }
        }

        // Gravity
        direction += Vec2::new(0.0, 1.0 * GRAVITY_STRENGTH);

        //
        direction = direction * time.delta_seconds();

        controller.translation = Some(direction);
    }
}

pub fn jump_timer_start(mut jump_timer: ResMut<JumpTimer>, time: Res<Time>) {
    jump_timer.timer.tick(time.delta());
}

pub fn air_dash_timer_start(mut air_dash_timer: ResMut<AirDash>, time: Res<Time>) {
    air_dash_timer.timer.tick(time.delta());
}

pub fn states(
    controllers: Query<&KinematicCharacterControllerOutput, With<Player>>,
    current_state: Res<State<MovementState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut jump_timer: ResMut<JumpTimer>,
    mut air_dash: ResMut<AirDash>,
) {
    for output in controllers.iter() {
        if current_state.0 == MovementState::Jumping
            && f32::trunc(output.desired_translation[1] * 100.0) / 100.0
                - f32::trunc(output.effective_translation[1] * 100.0) / 100.0
                > 0.1
            && (output.effective_translation[1] > 0.01 || output.effective_translation[1] < -0.01)
        {
            next_movement_state.set(MovementState::Falling);
            jump_timer.timer.reset();
            println!("State to Falling");
        }

        if keyboard_input.just_pressed(KeyCode::Space)
            && current_state.0 == MovementState::Idle
            && output.grounded
        {
            println!("State to Jumping");
            next_movement_state.set(MovementState::Jumping);
        }

        if jump_timer.timer.just_finished() && current_state.0 == MovementState::Jumping {
            jump_timer.timer.reset();
            println!("State to Falling");
            next_movement_state.set(MovementState::Falling);
        }

        if output.grounded
            && current_state.0 != MovementState::Idle
            && current_state.0 != MovementState::Jumping
        {
            next_movement_state.set(MovementState::Idle);
            air_dash.used = false;
            println!("State to Idle");
        }

        if output.grounded == false && current_state.0 == MovementState::Idle {
            next_movement_state.set(MovementState::Falling);
            println!("State to Falling");
        }

        if keyboard_input.just_pressed(KeyCode::LShift)
            && air_dash.used == false
            && (current_state.0 == MovementState::Jumping
                || current_state.0 == MovementState::Falling)
            && (keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D))
        {
            next_movement_state.set(MovementState::AirDash);
            jump_timer.timer.reset();
            air_dash.used = true;
            println!("State to AirDash");
        }

        if air_dash.timer.just_finished() && current_state.0 == MovementState::AirDash {
            next_movement_state.set(MovementState::Falling);
            air_dash.timer.reset();
            println!("State to Falling");
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_flame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(transform_player) = player_query.get_single() {
        let texture_handle = asset_server.load("animations/burning_loop_1.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 32.0), 8, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 1, last: 7 };

        let mut rotation = 0.0;

        if keyboard_input.pressed(KeyCode::D) {
            rotation = 1.59;
        }
        if keyboard_input.pressed(KeyCode::A) {
            rotation = -1.59;
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: Vec3::new(
                        transform_player.translation.x - 100.0,
                        transform_player.translation.y,
                        transform_player.translation.z,
                    ),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(3.0, 3.0, 3.0),
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn flame_follow_player(
    mut flame_query: Query<&mut Transform, (With<AnimationIndices>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<AnimationIndices>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(transform_player) = player_query.get_single() {
        if let Ok(mut transform_flame) = flame_query.get_single_mut() {
            if keyboard_input.pressed(KeyCode::D) {
                transform_flame.translation = Vec3::new(
                    transform_player.translation.x - 70.0,
                    transform_player.translation.y - 7.0,
                    transform_player.translation.z,
                );
            }
            if keyboard_input.pressed(KeyCode::A) {
                transform_flame.translation = Vec3::new(
                    transform_player.translation.x + 70.0,
                    transform_player.translation.y - 7.0,
                    transform_player.translation.z,
                );
            }
        }
    }
}

pub fn despawn_flame(mut cmds: Commands, flame_query: Query<Entity, With<AnimationIndices>>) {
    if let Ok(entity) = flame_query.get_single() {
        cmds.entity(entity).despawn()
    }
}

pub fn player_die_on_hit(
    mut cmds: Commands,
    bullet_query: Query<&KinematicCharacterControllerOutput, (With<EnemyBullet>, Without<Player>)>,
    player_query: Query<Entity, (With<Player>, Without<EnemyBullet>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        for bullet_output in bullet_query.iter() {
            if !bullet_output.collisions.is_empty() {
                if bullet_output.collisions[0].entity == player_entity {
                    // cmds.entity(player_entity).despawn();
                    // next_game_state.set(GameState::MainMenu)
                }
            }
        }
    }
}
