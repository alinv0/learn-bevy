use bevy::asset::AssetServer;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::prelude::{Commands, default, Res};

pub fn play_sound(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    path: String,) {
    commands.spawn(AudioBundle {
        source: asset_server.load(path),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}