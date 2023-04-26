mod systems;

use bevy::prelude::*;

use crate::GameState;

use systems::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_game_on_click.in_set(OnUpdate(GameState::MainMenu)));
    }
}
