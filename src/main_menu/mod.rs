use bevy::prelude::*;

use crate::main_menu::systems::transition_to_main_menu_state;

mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transition_to_main_menu_state);
    }
}

