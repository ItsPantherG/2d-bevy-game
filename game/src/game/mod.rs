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
        app.add_plugin(MapPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_system(despawn_player_on_fall.in_set(OnUpdate(GameState::Game)));
    }
}
