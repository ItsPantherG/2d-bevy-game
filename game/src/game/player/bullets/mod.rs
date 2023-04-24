mod components;
mod systems;

use bevy::prelude::*;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_bullet)
            .add_system(bullet_direction)
            .add_system(despawn_bullet_on_collision)
            .add_system(despawn_bullet_over_time);
    }
}
