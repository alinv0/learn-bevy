use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::systems::interactions::{interact_with_play_button, interact_with_quit_button, transition_to_main_menu_state};
use crate::main_menu::systems::layout::{despawn_main_menu, spawn_main_menu};

mod systems;
mod components;
mod styles;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(Update,
                (
                    interact_with_play_button,
                    interact_with_quit_button,
                ).run_if(in_state(AppState::MainMenu))
            )
            .add_systems(Update, transition_to_main_menu_state);
    }
}

