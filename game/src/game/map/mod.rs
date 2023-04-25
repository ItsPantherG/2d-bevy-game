mod resources;
mod systems;

use bevy::prelude::*;

use systems::*;

use self::resources::CurrentChunk;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentChunk>()
            .add_startup_system(spawn_map)
            .add_system(change_chunks)
            .add_system(generate_random_chunk);
    }
}
