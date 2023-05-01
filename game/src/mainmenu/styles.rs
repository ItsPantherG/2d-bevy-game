use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.3, 0.4);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.3, 0.2, 0.3);

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
  TextStyle {
    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    font_size: 16.0,
    color: Color::WHITE,
  }
}