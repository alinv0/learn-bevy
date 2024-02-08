use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::super::gameover::events::GameOver;
use super::super::player::components::Player;
use super::super::player::PLAYER_SIZE;
use super::super::score::components::Score;
use crate::util::audio::play_sound;
use crate::util::calculations::{detect_collision, get_boundaries, get_bounded_translation, get_random_bounded_coordinates, get_random_direction};

use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};
use super::components::Enemy;
use super::resources::EnemySpawnTimer;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let s = asset_server.clone();
        random_spawn(&mut commands, s, window);
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn random_spawn(commands: &mut Commands, asset_server: AssetServer, window: &Window) {
    let (x, y) = get_random_bounded_coordinates(window, ENEMY_SIZE);

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: get_random_direction(),
            },
        )
    );
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let (x_min, x_max, y_min, y_max) = get_boundaries(window, ENEMY_SIZE);

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            if random::<f32>() > 0.5 {
                play_sound(&mut commands, &asset_server, "audio/pluck_001.ogg".to_string());
            } else {
                play_sound(&mut commands, &asset_server, "audio/pluck_002.ogg".to_string());
            }
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let (x_min, x_max,
        y_min, y_max) = get_boundaries(window, ENEMY_SIZE);

    for mut transform in enemy_query.iter_mut() {
        transform.translation = get_bounded_translation(
            transform.translation,
            x_min, x_max,
            y_min, y_max,
        );
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            if detect_collision(player_transform, PLAYER_SIZE, enemy_transform, ENEMY_SIZE) {
                println!("Enemy hit the player! Game Over!");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver {
                    score: score.value,
                });
            }
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let (x, y) = get_random_bounded_coordinates(window, ENEMY_SIZE);

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: get_random_direction(),
                },
            )
        );
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}