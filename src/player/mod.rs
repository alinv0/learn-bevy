use bevy::app::Update;
use bevy::prelude::*;

use systems::*;

pub mod systems;
pub mod components;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update,
                         (
                             (
                                 player_movement,
                                 confine_player_movement
                             ).chain(),
                             player_hit_star, ));
    }
}
