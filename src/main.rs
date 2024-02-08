use bevy::prelude::*;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;


mod util;
mod camera;
mod game;
mod main_menu;


fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            MainMenuPlugin,))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}