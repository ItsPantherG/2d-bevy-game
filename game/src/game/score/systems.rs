use bevy::prelude::*;

use super::resources::*;
use crate::game::map::resources::*;

pub fn update_score(mut score: ResMut<PlayerScore>, current_chunk: Res<CurrentChunk>) {
    if score.value != current_chunk.value as u32 {
        score.value = current_chunk.value as u32;
    }
}

pub fn show_final_score_cmd(score: Res<PlayerScore>) {
    println!("{}", score.value)
}
