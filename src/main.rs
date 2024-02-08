use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::gameover::GameOverPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;

mod star;
mod score;
mod gameover;
mod util;
mod camera;
mod enemy;
mod player;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            GameOverPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ScorePlugin,
            StarPlugin))
        .run();
}