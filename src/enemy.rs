use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, default, Entity, Query, Res, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use crate::general::{detect_collision, get_boundaries, get_bounded_translation, get_random_bounded_coordinates};
use crate::player::{Player, PLAYER_SIZE};

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let (x, y) = get_random_bounded_coordinates(window, ENEMY_SIZE);

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    }
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

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    let sound1 = asset_server.load("audio/pluck_001.ogg");
    let sound2 = asset_server.load("audio/pluck_002.ogg");

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound = if random::<f32>() > 0.5 {
                sound1.clone()
            } else {
                sound2.clone()
            };
            commands.spawn(AudioBundle {
                source: sound,
                settings: PlaybackSettings::ONCE,
                ..default()
            });
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
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
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
            }
        }
    }
}

