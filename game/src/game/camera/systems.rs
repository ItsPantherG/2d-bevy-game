use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::game::player::compoments::*;

pub fn spawn_camera(mut cmds: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    cmds.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 999.0),
            ..default()
        },
        Camera,
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(mut transform_camera) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            let player_translation_x = player_transform.translation[0];

            if transform_camera.translation[0] != player_translation_x {
                transform_camera.translation[0] = player_translation_x
            }
        }
    }
}
