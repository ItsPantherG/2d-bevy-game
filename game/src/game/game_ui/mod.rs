pub mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::game::InGameState;
use crate::GameState;
use systems::layout::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_ui.in_schedule(OnEnter(GameState::Game)))
            .add_system(
                update_score
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(OnUpdate(InGameState::Playing)),
            )
            .add_system(despawn_game_ui.in_schedule(OnExit(GameState::Game)));
    }
}
