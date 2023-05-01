mod game;
mod mainmenu;

use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin };

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;

use game::GamePlugin;
use mainmenu::MainMenuPlugin;

fn main() {
  App::new()
    // Add State
    .add_state::<GameState>()
    // Add plugins imported
    .add_plugins(
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          title: "bevy game".into(),
          resolution: (900.0, 700.0).into(),
          present_mode: bevy::window::PresentMode::Immediate,
          ..default()
        }),
        ..default()
      })
    )
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    // Add Plugins
    .add_plugin(MainMenuPlugin)
    .add_plugin(GamePlugin)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
  #[default]
  MainMenu,
  Game,
  Pauze,
  GameOver,
}