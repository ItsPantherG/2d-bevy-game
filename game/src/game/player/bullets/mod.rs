mod components;
mod systems;

use bevy::prelude::*;

use crate::GameState;
use super::InGameState;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      (
        // spawn_bullet,
         bullet_direction, 
         despawn_bullet_on_collision, 
        despawn_bullet_over_time)
        .in_set(OnUpdate(InGameState::Playing))
        .in_set(OnUpdate(GameState::Game))
    );
  }
}