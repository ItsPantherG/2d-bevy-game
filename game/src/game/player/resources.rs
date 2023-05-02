use bevy::prelude::*;

#[derive(Resource)]
pub struct JumpTimer {
  pub timer: Timer,
}

impl Default for JumpTimer {
  fn default() -> Self {
    JumpTimer {
      timer: Timer::from_seconds(0.5, TimerMode::Repeating),
    }
  }
}

#[derive(Resource)]
pub struct AirDash {
  pub timer: Timer,
  pub used: bool,
}

impl Default for AirDash {
  fn default() -> Self {
    AirDash {
      timer: Timer::from_seconds(0.15, TimerMode::Repeating),
      used: false,
    }
  }
}