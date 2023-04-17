use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_state::<MovementState>()
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .init_resource::<JumpTimer>()
        .init_resource::<AirDash>()
        .add_startup_system(spawn_game)
        .add_system(player_move)
        .add_system(states)
        .add_system(jump_timer_start.in_set(OnUpdate(MovementState::Jumping)))
        .add_system(air_dash_timer_start.in_set(OnUpdate(MovementState::AirDash)))
        .add_system(camera_follow_player)
        .run()
}

pub const PLAYER_SPEED: f32 = 500.0;
pub const GRAVITY: f32 = 4250.;
pub const JUMP_STRENGTH: f32 = 5000.0;
pub const AIR_DASH_SPEED: f32 = 1500.0;
pub const GRAVITY_STRENGTH: f32 = -1000.0;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct JumpTimer {
    timer: Timer,
}

impl Default for JumpTimer {
    fn default() -> Self {
        JumpTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct AirDash {
    timer: Timer,
    used: bool,
}

impl Default for AirDash {
    fn default() -> Self {
        AirDash {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            used: false,
        }
    }
}

pub fn spawn_game(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    cmds.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 999.0),
            ..default()
        },
        Camera,
    ));

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
    cmds.spawn(Collider::cuboid(217.5, 40.0))
        .insert(SpriteBundle {
            transform: Transform::from_xyz(800.0, 100.0, 1.0),
            texture: asset_server.load("sprites/ground.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(455.0, 100.0)),
                ..default()
            },
            ..default()
        });

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
        up: Vec2::Y,
        snap_to_ground: Some(CharacterLength::Relative(0.2)),
        min_slope_slide_angle: 50.0_f32.to_radians(),
        ..default()
    })
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::capsule_x(20.0, 20.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(GravityScale(5.0));
}

pub fn player_move(
    mut controllers: Query<&mut KinematicCharacterController>,
    controllers_output: Query<&KinematicCharacterControllerOutput>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    current_state: Res<State<MovementState>>,
    jump_timer: Res<JumpTimer>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec2::ZERO;

        // Left Right
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec2::new(-1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec2::new(1.0, 0.0);
        }

        // Normalize and set player speed
        if direction.length() > 0.0 {
            direction = direction.normalize_or_zero() * PLAYER_SPEED
        }

        // set jump speed
        if current_state.0 == MovementState::Jumping {
            direction += Vec2::new(0.0, jump_timer.timer.remaining_secs()) * JUMP_STRENGTH
        }

        // air dash
        if current_state.0 == MovementState::AirDash {
            for output in controllers_output.iter() {
                let player_movement = output.effective_translation[0];
                if player_movement > 0.0 {
                    direction += Vec2::new(1.0, -GRAVITY_STRENGTH / AIR_DASH_SPEED) * AIR_DASH_SPEED
                }
                if player_movement < 0.0 {
                    direction +=
                        Vec2::new(-1.0, -GRAVITY_STRENGTH / AIR_DASH_SPEED) * AIR_DASH_SPEED
                }
            }
        }

        // Gravity
        direction += Vec2::new(0.0, 1.0 * GRAVITY_STRENGTH);

        //
        direction = direction * time.delta_seconds();

        controller.translation = Some(direction)
    }
}

pub fn camera_follow_player(
    current_state: Res<State<MovementState>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    controllers: Query<&KinematicCharacterControllerOutput>,
    time: Res<Time>,
) {
    if let Ok(mut transform_camera) = camera_query.get_single_mut() {
        for controller in controllers.iter() {
            let effective_x = f32::trunc(controller.effective_translation[0] * 100.0) / 100.0;
            let desired_x = f32::trunc(controller.desired_translation[0] * 100.0) / 100.0;

            let delta = controller.effective_translation[0];
            let mut direction = Vec3::ZERO;

            direction += Vec3::new(effective_x, 0.0, 0.0);

            if current_state.0 == MovementState::AirDash {
                transform_camera.translation +=
                    direction.normalize_or_zero() * AIR_DASH_SPEED * time.delta_seconds()
            }

            transform_camera.translation +=
                direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds()
        }
    }
}

pub fn jump_timer_start(mut jump_timer: ResMut<JumpTimer>, time: Res<Time>) {
    jump_timer.timer.tick(time.delta());
}

pub fn air_dash_timer_start(mut air_dash_timer: ResMut<AirDash>, time: Res<Time>) {
    air_dash_timer.timer.tick(time.delta());
}

pub fn states(
    controllers: Query<&KinematicCharacterControllerOutput>,
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
            next_movement_state.set(MovementState::Falling)
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
            println!("State to Falling")
        }

        if keyboard_input.just_pressed(KeyCode::Space)
            && air_dash.used == false
            && (current_state.0 == MovementState::Jumping
                || current_state.0 == MovementState::Falling)
        {
            next_movement_state.set(MovementState::AirDash);
            jump_timer.timer.reset();
            air_dash.used = true;
            println!("State to AirDash")
        }

        if air_dash.timer.just_finished() && current_state.0 == MovementState::AirDash {
            next_movement_state.set(MovementState::Falling);
            air_dash.timer.reset();
            println!("State to Falling")
        }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MovementState {
    #[default]
    Falling,
    Idle,
    Jumping,
    AirDash,
}
