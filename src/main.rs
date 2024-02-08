use bevy::prelude::*;

use crate::enemy::{confine_enemy_movement, enemy_hit_player, enemy_movement, EnemySpawnTimer, spawn_enemies, spawn_enemies_over_time, tick_enemy_spawn_timer, update_enemy_direction};
use crate::gameover::{exit_game, GameOver, handle_game_over};
use crate::player::{confine_player_movement, player_hit_star, player_movement, spawn_player};
use crate::star::{spawn_stars, spawn_stars_over_time, StarSpawnTimer, tick_star_spawn_timer};
use crate::startup::spawn_camera;
use crate::score::{high_scores_updated, HighScores, Score, update_high_scores, update_score};

mod startup;
mod general;
mod enemy;
mod player;
mod star;
mod score;
mod gameover;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()
        .add_event::<GameOver>()
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
            tick_enemy_spawn_timer,
            spawn_enemies_over_time,
            exit_game,
            handle_game_over,
            update_high_scores,
            high_scores_updated,
        ))
        .run();
}