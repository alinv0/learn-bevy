use bevy::app::{App, Update};
use bevy::prelude::*;

use resources::*;
use systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub mod systems;
pub mod resources;
pub mod components;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const START_SPAWN_INTERVAL: f32 = 1.0;


pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(Update, (
                tick_star_spawn_timer,
                spawn_stars_over_time,
            ).run_if(in_state(AppState::Game)).run_if(in_state(SimulationState::Running)));
    }
}