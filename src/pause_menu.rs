use std::ops::Not;

use bevy::prelude::*;

use crate::input_translation::GameInput;

pub struct PauseMenuPlugin;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum PausedState {
    Unpaused,
    Paused,
}

impl Not for PausedState {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == PausedState::Unpaused {
            PausedState::Paused
        } else {
            PausedState::Unpaused
        }
    }
}

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state::<PausedState>(PausedState::Unpaused);
        app.add_systems(Update, check_for_pause);
    }
}

fn check_for_pause(
    mut event_reader: EventReader<GameInput>,
    current_state: Res<State<PausedState>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    for input in event_reader.read() {
        if *input == GameInput::Start {
            next_state.set(!*current_state.get());
            return;
        }
    }
}
