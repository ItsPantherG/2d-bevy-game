mod components;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            // Add Systems
            .add_system(camera_follow_player.in_set(OnUpdate(GameState::Game)));
    }
}
