use bevy::prelude::*;
use bevy::window::WindowMode;
use constants::WINDOW_DIMENSIONS;
use game::GamePlugin;
use input_translation::InputTranslationPlugin;

use crate::background::BackgroundPlugin;
use crate::camera::CameraPlugin;
use crate::constants::GAME_NAME;

mod animation;
mod background;
mod camera;
mod constants;
mod game;
mod input_translation;
mod physics;
mod player;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum GameState {
    Paused,
    Playing,
}

fn main() {
    let mut app = App::new();
    let window_mode = if cfg!(debug_assertions) {
        WindowMode::Windowed
    } else {
        // Current and Primary do not work on Wayland
        WindowMode::BorderlessFullscreen(MonitorSelection::Index(0))
    };
    // Bevy Plugins
    let window = Window {
        title: GAME_NAME.into(),
        name: Some(GAME_NAME.into()),
        resolution: WINDOW_DIMENSIONS.into(),
        resizable: false,
        mode: window_mode,
        ..default()
    };
    // Set default_nearewast to prevent blurry sprits
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }));
    // My Plugins
    app.add_plugins((CameraPlugin, BackgroundPlugin, GamePlugin, InputTranslationPlugin));
    app.insert_state(GameState::Paused);
    app.insert_resource(Time::<Fixed>::from_hz(96.0));
    app.run();
}
