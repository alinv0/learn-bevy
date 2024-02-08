use bevy::app::{App, Startup};
use bevy::prelude::Plugin;

use systems::spawn_camera;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}