use bevy::prelude::*;

use crate::AppState;

use super::events::GameOver;

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Your score: {}", event.score.to_string());
        next_app_state.set(AppState::GameOver);
    }
}