use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::general::get_random_bounded_coordinates;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const START_SPAWN_INTERVAL: f32 = 1.0;

#[derive(Component)]
pub struct Star {}

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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let (x, y) = get_random_bounded_coordinates(window, STAR_SIZE);

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            )
        );
    }
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let (x, y) = get_random_bounded_coordinates(window, STAR_SIZE);

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            )
        );
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}