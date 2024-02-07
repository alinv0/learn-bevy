use bevy::prelude::*;

use crate::enemy::{confine_enemy_movement, enemy_hit_player, enemy_movement, spawn_enemies, update_enemy_direction};
use crate::player::{confine_player_movement, player_hit_star, player_movement, spawn_player};
use crate::star::{spawn_stars, spawn_stars_over_time, StarSpawnTimer, tick_star_spawn_timer};
use crate::startup::spawn_camera;
use crate::ui::{Score, update_score};

mod startup;
mod general;
mod enemy;
mod player;
mod star;
mod ui;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_enemies,
            spawn_stars
        ))
        .add_systems(Update, (
            player_movement,
            confine_player_movement,
            player_hit_star,
            enemy_movement,
            update_enemy_direction,
            confine_enemy_movement,
            enemy_hit_player,
            update_score,
            tick_star_spawn_timer,
            spawn_stars_over_time,
        ))
        .run();
}