pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use systems::*;

use crate::GameState;

use self::resources::CurrentChunk;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Resources
            .init_resource::<CurrentChunk>()
            // On Enter State
            .add_system(spawn_map.in_schedule(OnEnter(GameState::Game)))
            // Add Systems
            .add_systems((change_chunks, generate_random_chunk).in_set(OnUpdate(GameState::Game)))
            .add_system(despawn_map.in_schedule(OnExit(GameState::Game)));
    }
}
