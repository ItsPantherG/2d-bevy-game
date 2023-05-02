mod bullets;
pub mod compoments;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::GameState;

use bullets::*;
use resources::*;
use systems::*;

use super::InGameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      // Add State
      .add_state::<MovementState>()
      // Add plugins
      .add_plugin(BulletPlugin)
      // Add Resources
      .init_resource::<JumpTimer>()
      .init_resource::<AirDash>()
      // On Enter State
      .add_system(spawn_player.in_schedule(OnEnter(GameState::Game)))
      .add_system(
        spawn_flame.in_schedule(OnEnter(MovementState::AirDash)).in_set(OnUpdate(GameState::Game))
      )
      // Add Systems
      .add_systems(
        (
          player_move,
          states,
          jump_timer_start.in_set(OnUpdate(MovementState::Jumping)),
          air_dash_timer_start.in_set(OnUpdate(MovementState::AirDash)),
          flame_follow_player.in_set(OnUpdate(MovementState::AirDash)),
          animate_sprite,
        )
          .in_set(OnUpdate(InGameState::Playing))
          .in_set(OnUpdate(GameState::Game))
      )
      // On Exit State
      .add_system(despawn_flame.in_schedule(OnExit(MovementState::AirDash)));
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