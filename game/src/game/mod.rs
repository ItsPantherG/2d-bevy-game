pub mod camera;
pub mod map;
pub mod player;
mod systems;

use bevy::prelude::*;

use super::GameState;

use camera::*;
use map::*;
use player::*;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<InGameState>()
      .add_plugin(MapPlugin)
      .add_plugin(CameraPlugin)
      .add_plugin(PlayerPlugin)
      .add_system(change_paused_game_state.in_set(OnUpdate(GameState::Game)))
      .add_system(despawn_player_on_fall.in_set(OnUpdate(GameState::Game)));
  }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGameState {
  #[default]
  Playing,
  Paused,
}