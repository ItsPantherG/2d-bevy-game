mod bullets;
pub mod compoments;
pub mod resources;
mod systems;

use bevy::prelude::*;

use bullets::*;
use resources::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MovementState>()
            .add_plugin(BulletPlugin)
            .init_resource::<JumpTimer>()
            .init_resource::<AirDash>()
            .add_startup_system(spawn_player)
            .add_system(player_move)
            .add_system(states)
            .add_system(jump_timer_start.in_set(OnUpdate(MovementState::Jumping)))
            .add_system(air_dash_timer_start.in_set(OnUpdate(MovementState::AirDash)))
            .add_system(spawn_flame.in_schedule(OnEnter(MovementState::AirDash)))
            .add_system(flame_follow_player.in_set(OnUpdate(MovementState::AirDash)))
            .add_system(despawn_flame.in_schedule(OnExit(MovementState::AirDash)))
            .add_system(animate_sprite);
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
