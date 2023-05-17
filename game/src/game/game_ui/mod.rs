pub mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::game::InGameState;
use crate::GameState;
use systems::{interactions::*, layout::*};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_ui.in_schedule(OnEnter(GameState::Game)))
            .add_systems(
                (update_score, update_health_bar)
                    .in_set(OnUpdate(GameState::Game))
                    .in_set(OnUpdate(InGameState::Playing)),
            )
            .add_systems(
                (despawn_game_ui, despawn_health_bar).in_schedule(OnExit(GameState::Game)),
            );
    }
}
