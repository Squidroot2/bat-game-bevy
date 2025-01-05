use bevy::prelude::*;

use crate::{
    animation::{animate_sprites, direct_sprites},
    audio::SoundEffectSystem,
    input_translation::{GameInput, InputTranslationSystem},
    pause_menu::PausedState,
    physics::{add_friction, add_gravity, move_with_velocity, wrap_position},
    player::{handle_input, reset_player, spawn_player, PlayerFlapped, PlayerScreetched},
    GameState,
};

#[derive(Event, Default)]
pub struct Reset;

#[derive(Event)]
pub struct EnemyEaten;

#[derive(SystemSet, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GameplaySystem;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Reset>();
        app.add_event::<PlayerFlapped>();
        app.add_event::<PlayerScreetched>();
        app.add_event::<EnemyEaten>();
        app.add_systems(
            PreUpdate,
            check_game_start.after(InputTranslationSystem).run_if(in_state(GameState::Ready)),
        );
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
                .in_set(GameplaySystem)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(PausedState::Unpaused)),
        );
        app.add_systems(Update, reset_player);
        app.configure_sets(Update, GameplaySystem.before(SoundEffectSystem));
    }
}

fn check_game_start(mut input_reader: EventReader<GameInput>, mut next_state: ResMut<NextState<GameState>>) {
    for input in input_reader.read() {
        match input {
            GameInput::Screetch | GameInput::Flap => next_state.set(GameState::Playing),
            GameInput::Start => continue,
        }
    }
}
