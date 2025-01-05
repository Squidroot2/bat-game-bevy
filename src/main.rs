use bevy::prelude::*;
use bevy::window::WindowMode;
use pause_menu::PauseMenuPlugin;

use crate::audio::SoundPlugin;
use crate::background::BackgroundPlugin;
use crate::camera::CameraPlugin;
use crate::constants::GAME_NAME;
use crate::constants::WINDOW_DIMENSIONS;
use crate::game::GamePlugin;
use crate::input_translation::InputTranslationPlugin;

mod animation;
mod audio;
mod background;
mod camera;
mod constants;
mod game;
mod input_translation;
mod pause_menu;
mod physics;
mod player;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum GameState {
    Ready,
    Gameover,
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
    // Set default_nearest to prevent blurry sprits
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }));
    // My Plugins
    app.add_plugins((
        CameraPlugin,
        BackgroundPlugin,
        GamePlugin,
        InputTranslationPlugin,
        SoundPlugin,
        PauseMenuPlugin,
    ));
    app.insert_state(GameState::Ready);
    app.run();
}
