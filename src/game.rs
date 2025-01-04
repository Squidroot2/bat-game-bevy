use bevy::prelude::*;

use crate::{
    animation::{animate_sprites, direct_sprites},
    input_translation::GameInput,
    physics::{add_friction, add_gravity, move_with_velocity, wrap_position},
    player::{handle_input, spawn_player},
    GameState,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, check_pause);
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                handle_input.before(move_with_velocity),
                animate_sprites.after(handle_input),
                direct_sprites.after(handle_input),
                add_gravity.before(move_with_velocity),
                move_with_velocity,
                add_friction.after(move_with_velocity),
                wrap_position.after(move_with_velocity),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn check_pause(mut reader: EventReader<GameInput>, state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    for input in reader.read() {
        if *input == GameInput::Start {
            match state.get() {
                GameState::Playing => next_state.set(GameState::Paused),
                GameState::Paused => next_state.set(GameState::Playing),
            }
        }
        if *input == GameInput::Flap && *state.get() == GameState::Paused {
            next_state.set(GameState::Playing)
        }
    }
}
