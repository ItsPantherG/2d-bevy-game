use bevy::prelude::*;

use crate::GameState;

use crate::game::map::components::*;
use crate::game::map::resources::*;
use crate::game::player::compoments::*;

pub fn despawn_player_on_fall(
    mut cmds: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    map_query: Query<Entity, With<Chunk>>,
    mut current_chunk: ResMut<CurrentChunk>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, transform_player)) = player_query.get_single() {
        if transform_player.translation.y < -10.0 {
            for map_entity in map_query.iter() {
                cmds.entity(map_entity).despawn()
            }
            cmds.entity(entity).despawn();
            current_chunk.value = 0.0;
            next_game_state.set(GameState::MainMenu)
        }
    }
}
