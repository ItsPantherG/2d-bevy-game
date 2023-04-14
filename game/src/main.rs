use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_state::<MovementState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .init_resource::<JumpTimer>()
        .add_startup_system(setup)
        .add_system(player_move)
        .add_system(jump)
        .add_system(jump_timer_start.in_set(OnUpdate(MovementState::Jumping)))
        .run()
}

pub const PLAYER_SPEED: f32 = 300.0;
pub const GRAVITY: f32 = 4250.;
pub const JUMP_STRENGTH: f32 = 5000.0;
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

pub fn setup(
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

    for i in 0..3 {
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
            transform: Transform::from_xyz(200.0, 250.0, 1.0),
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

        // Gravity
        direction += Vec2::new(0.0, 1.0 * GRAVITY_STRENGTH);

        //
        direction = direction * time.delta_seconds();

        controller.translation = Some(direction)
    }
}

pub fn jump_timer_start(mut jump_timer: ResMut<JumpTimer>, time: Res<Time>) {
    jump_timer.timer.tick(time.delta());
}

pub fn jump(
    controllers: Query<&KinematicCharacterControllerOutput>,
    current_state: Res<State<MovementState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut jump_timer: ResMut<JumpTimer>,
) {
    for output in controllers.iter() {
        if current_state.0 == MovementState::Jumping
            && f32::trunc(output.effective_translation[1] * 100.0) / 100.0
                != f32::trunc(output.desired_translation[1] * 100.0) / 100.0
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
            println!("State to Falling");
            next_movement_state.set(MovementState::Falling)
        }

        if output.grounded
            && current_state.0 != MovementState::Idle
            && current_state.0 != MovementState::Jumping
        {
            next_movement_state.set(MovementState::Idle);
            println!("State to Idle");
        }

        if output.grounded == false && current_state.0 == MovementState::Idle {
            next_movement_state.set(MovementState::Falling);
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
}
