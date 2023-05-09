use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyShootTimer {
    pub timer: Timer,
}

impl Default for EnemyShootTimer {
    fn default() -> Self {
        EnemyShootTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
