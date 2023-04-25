use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentChunk {
    pub value: f32,
}

impl Default for CurrentChunk {
    fn default() -> Self {
        CurrentChunk { value: -1.0 }
    }
}
