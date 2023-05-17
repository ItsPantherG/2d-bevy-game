use bevy::prelude::*;

use super::super::components::*;
use crate::game::player::compoments::*;
use crate::game::score::resources::*;

pub fn update_health_bar(
    mut health_bar_query: Query<(&mut Transform, &mut Sprite), (With<HealthBar>, Without<Player>)>,
    player_query: Query<(&Transform, &Player), (With<Player>, Without<HealthBar>)>,
) {
    if let Ok((mut health_bar_transform, mut sprite)) = health_bar_query.get_single_mut() {
        if let Ok((player_transform, player)) = player_query.get_single() {
            let translation = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y + 40.0,
                player_transform.translation.z,
            );

            health_bar_transform.translation = translation;

            let health = player.health as f32;

            sprite.custom_size = Some(Vec2::new(80.0 * (health / 100.0), 8.0))
        }
    }
}

pub fn update_score(score: Res<PlayerScore>, mut score_query: Query<&mut Text, With<ShowScore>>) {
    if score.is_changed() {
        if let Ok(mut text) = score_query.get_single_mut() {
            text.sections[0].value = format!("score: {}", score.value.to_string())
        }
    }
}
