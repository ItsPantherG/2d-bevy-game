use bevy::prelude::*;

use super::super::components::*;

pub fn spawn_game_ui(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let _game_ui_entity = build_game_ui(&mut cmds, &asset_server);
    let _player_health_bar = build_player_health_bar(&mut cmds, &asset_server);
}

pub fn despawn_game_ui(mut cmds: Commands, game_ui_query: Query<Entity, With<GameUi>>) {
    if let Ok(entity) = game_ui_query.get_single() {
        cmds.entity(entity).despawn_recursive()
    }
}
pub fn despawn_health_bar(mut cmds: Commands, health_bar_query: Query<Entity, With<HealthBar>>) {
    if let Ok(entity) = health_bar_query.get_single() {
        cmds.entity(entity).despawn_recursive()
    }
}

pub fn build_game_ui(cmds: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_ui_entity = cmds
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            GameUi {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                        },
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                        gap: Size::new(Val::Px(10.0), Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "NONE fps",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 16.0,
                                        color: Color::ALICE_BLUE.into(),
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ShowFPS {},
                    ));
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Score: 0",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 16.0,
                                        color: Color::ALICE_BLUE.into(),
                                    },
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ShowScore {},
                    ));
                });
        })
        .id();

    game_ui_entity
}

pub fn build_player_health_bar(cmds: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let player_health_bar_entity = cmds
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(100.0, 100.0, 101.0),
                    ..default()
                },
                texture: asset_server.load("sprites/icons/BarV3_ProgressBar.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80.0, 8.0)),
                    ..default()
                },
                ..default()
            },
            HealthBar {},
        ))
        .insert(Name::new("player_health"))
        .id();

    player_health_bar_entity
}
