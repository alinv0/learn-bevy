use bevy::asset::AssetServer;
use bevy::prelude::{Commands, default, Query, Res, ResMut, SpriteBundle, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::star::{NUMBER_OF_STARS, STAR_SIZE};
use crate::util::calculations::get_random_bounded_coordinates;

use super::components::Star;
use super::resources::StarSpawnTimer;

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