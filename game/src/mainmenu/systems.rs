use bevy::prelude::*;

use crate::GameState;

pub fn start_game_on_click(
    mouse_input: Res<Input<MouseButton>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        next_game_state.set(GameState::Game)
    }
}
