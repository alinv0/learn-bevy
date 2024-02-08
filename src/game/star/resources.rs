use bevy::prelude::*;

use super::START_SPAWN_INTERVAL;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(START_SPAWN_INTERVAL, TimerMode::Repeating)
        }
    }
}