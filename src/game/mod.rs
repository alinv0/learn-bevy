use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::game::enemy::EnemyPlugin;
use crate::game::gameover::GameOverPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::score::ScorePlugin;
use crate::game::star::StarPlugin;
use systems::*;
use crate::AppState;

pub mod enemy;
pub mod player;
pub mod gameover;
pub mod score;
pub mod star;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins((
                CameraPlugin,
                GameOverPlugin,
                PlayerPlugin,
                EnemyPlugin,
                ScorePlugin,
                StarPlugin))
            .add_systems(Update, (
                exit_game,
                transition_to_game_state,
                toggle_simulation.run_if(in_state(AppState::Game))));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}