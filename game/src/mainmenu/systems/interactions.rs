use bevy::prelude::*;
use bevy::app::AppExit;

use crate::GameState;
use super::super::components::*;
use super::super::styles::{ HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR };

pub fn play_button_interaction(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<PlayButton>)
  >,
  mut next_game_state: ResMut<NextState<GameState>>
) {
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *background_color = PRESSED_BUTTON_COLOR.into();
        next_game_state.set(GameState::Game)
      }
      Interaction::Hovered => {
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}

pub fn quit_button_interaction(
  mut button_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<QuitButton>)
  >,
  mut app_exit_event_writer: EventWriter<AppExit>
) {
  if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
    match *interaction {
      Interaction::Clicked => {
        *background_color = PRESSED_BUTTON_COLOR.into();
        app_exit_event_writer.send(AppExit)
      }
      Interaction::Hovered => {
        *background_color = HOVERED_BUTTON_COLOR.into();
      }
      Interaction::None => {
        *background_color = NORMAL_BUTTON_COLOR.into();
      }
    }
  }
}