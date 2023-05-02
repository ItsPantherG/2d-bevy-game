mod systems;
pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::GameState;

use systems::layout::*;
use systems::interactions::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      // On Enter Game State
      .add_system(spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)))

      // Systems
      .add_system(play_button_interaction.in_set(OnUpdate(GameState::MainMenu)))
      .add_system(quit_button_interaction.in_set(OnUpdate(GameState::MainMenu)))

      // On Exit Game State
      .add_system(despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)));
  }
}