use bevy::prelude::{App, Plugin, Update};

use components::*;
use systems::*;

pub mod systems;
pub mod components;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, (
                update_score,
                update_high_scores,
                high_scores_updated,
            ));
    }
}