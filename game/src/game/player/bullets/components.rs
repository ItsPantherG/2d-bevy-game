use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub timer: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            direction: Vec2::ZERO,
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}
