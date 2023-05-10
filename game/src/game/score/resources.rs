use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerScore {
    pub value: u32,
}

impl Default for PlayerScore {
    fn default() -> Self {
        PlayerScore { value: 0 }
    }
}
