mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use crate::game::InGameState;
use crate::GameState;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerScore>()
            .add_system(
                update_score
                    .in_set(OnUpdate(InGameState::Playing))
                    .in_set(OnUpdate(GameState::Game)),
            )
            .add_system(show_final_score_cmd.in_schedule(OnExit(GameState::Game)));
    }
}
