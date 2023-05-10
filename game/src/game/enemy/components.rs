use bevy::prelude::*;

#[derive(Component)]
pub struct EnemySaltThrower {}

#[derive(Component)]
pub struct EnemyBullet {
    pub desired_direction: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBulletTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBulletDespawnTimer(pub Timer);
