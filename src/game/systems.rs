use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;
use crate::game::SimulationState;

pub fn pause_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Paused");
        }
        if **simulation_state == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Running");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if **app_state != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("AppState: Game");
        }
    }
}