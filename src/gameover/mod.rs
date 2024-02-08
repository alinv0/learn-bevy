use bevy::app::App;
use bevy::prelude::{Plugin, Update};

use events::*;
use systems::*;

pub mod systems;
pub mod events;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_systems(Update, (
                exit_game,
                handle_game_over,
            ));
    }
}