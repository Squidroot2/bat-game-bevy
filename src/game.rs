use bevy::prelude::*;

use crate::{input::UserInput, GameState};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, check_pause);
    }
}

fn check_pause(mut reader: EventReader<UserInput>, state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    for input in reader.read() {
        if *input == UserInput::Start {
            match state.get() {
                GameState::Playing => next_state.set(GameState::Paused),
                GameState::Paused => next_state.set(GameState::Playing),
            }
        }
        if *input == UserInput::Flap && *state.get() == GameState::Paused {
            next_state.set(GameState::Playing)
        }
    }
}
