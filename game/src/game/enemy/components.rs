use bevy::prelude::*;

#[derive(Component)]
pub struct EnemySaltThrower {}

#[derive(Component)]
pub struct EnemyBullet {
    pub desired_direction: f32,
    pub is_hit_player: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBulletTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBulletDespawnTimer(pub Timer);
