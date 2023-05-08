use bevy::prelude::*;

#[derive(Component)]
pub struct EnemySaltThrower {}

#[derive(Component)]
pub struct EnemyBullet {
    pub dirction: i32,
    pub velocity_y: f32,
}
