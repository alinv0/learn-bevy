use bevy::prelude::*;

use components::*;
use systems::*;

use crate::AppState;

pub mod systems;
pub mod components;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), insert_score)
            .add_systems(Update, (
                update_score.run_if(in_state(AppState::Game)),
                update_high_scores.run_if(in_state(AppState::Game)),
                high_scores_updated,
            ));
    }
}