use bevy::prelude::*;

use super::super::components::*;
use super::super::styles::*;

pub fn spawn_main_menu(mut cmds: Commands, asset_server: Res<AssetServer>) {
  let _main_menu_entity = build_main_menu(&mut cmds, &asset_server);
}

pub fn despawn_main_menu(mut cmds: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
  if let Ok(entity) = main_menu_query.get_single() {
    cmds.entity(entity).despawn_recursive()
  }
}

pub fn build_main_menu(cmds: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
  let main_menu_entity =
    // Window width size parent
    cmds
      .spawn((
        NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Row,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: Color::DARK_GRAY.into(),
          ..default()
        },
        MainMenu {},
      ))

      //Center NodeBundle With buttons

      .with_children(|parent| {
        parent
          .spawn(NodeBundle {
            style: Style {
              flex_direction: FlexDirection::Column,
              align_items: AlignItems::Center,
              justify_content: JustifyContent::Center,
              size: Size::new(Val::Px(360.0), Val::Px(600.0)),
              border: UiRect::all(Val::Px(2.0)),
              gap: Size::new(Val::Px(40.0), Val::Px(40.0)),
              ..default()
            },
            ..default()
          })
          .with_children(|parent| {
            //Text Title
            parent.spawn(TextBundle {
              text: Text {
                sections: vec![
                  TextSection::new("Bevy Platformer", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE.into(),
                  })
                ],
                alignment: TextAlignment::Center,
                ..default()
              },
              ..default()
            });

            // Play Button
            parent
              .spawn((
                ButtonBundle {
                  style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(40.0)),
                    ..default()
                  },
                  background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                  ..default()
                },
                PlayButton {},
              ))
              .with_children(|parent| {
                parent.spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new("Play", get_button_text_style(&asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
              });

            // Quit Button
            parent
              .spawn((
                ButtonBundle {
                  style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(200.0), Val::Px(40.0)),
                    ..default()
                  },
                  background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                  ..default()
                },
                QuitButton {},
              ))
              .with_children(|parent| {
                parent.spawn(TextBundle {
                  text: Text {
                    sections: vec![TextSection::new("Quit", get_button_text_style(&asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                  },
                  ..default()
                });
              });
          });
      })
      .id();

  main_menu_entity
}