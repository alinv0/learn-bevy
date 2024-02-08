use bevy::prelude::*;

use crate::AppState;

use super::events::GameOver;

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Your score: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}