mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::GameState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyShootTimer>()
            .add_system(spawn_emeny_salt_thrower.in_set(OnUpdate(GameState::Game)))
            .add_systems(
                (
                    enemy_shoot_player,
                    despawn_enemy_bullet_on_collision,
                    enemy_bullet_direction,
                    start_enemy_bullet_timer,
                )
                    .in_set(OnUpdate(GameState::Game)),
            );
    }
}
