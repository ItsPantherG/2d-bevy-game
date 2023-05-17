use bevy::prelude::*;

use crate::GameState;

use crate::game::map::components::*;
use crate::game::map::resources::*;
use crate::game::player::compoments::*;
use crate::game::InGameState;

pub fn change_paused_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    current_in_game_state: Res<State<InGameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
) {
    if current_game_state.0 == GameState::Game {
        if keyboard_input.just_pressed(KeyCode::Escape)
            && current_in_game_state.0 != InGameState::Paused
        {
            next_in_game_state.set(InGameState::Paused);
            println!("Set InGameState To Paused");
        }
        if keyboard_input.just_pressed(KeyCode::Escape)
            && current_in_game_state.0 == InGameState::Paused
        {
            next_in_game_state.set(InGameState::Playing);
            println!("Set InGameState To Playing")
        }
    }
}

pub fn player_die(
    mut cmds: Commands,
    player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    map_query: Query<Entity, With<Chunk>>,
    mut current_chunk: ResMut<CurrentChunk>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, transform_player, player)) = player_query.get_single() {
        if transform_player.translation.y < -500.0 || player.health <= 0 {
            for map_entity in map_query.iter() {
                cmds.entity(map_entity).despawn();
            }
            cmds.entity(entity).despawn();
            current_chunk.value = 0.0;
            next_game_state.set(GameState::MainMenu)
        }
    }
}
