use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;

mod camera;
mod player;

fn main() {
    let mut app = App::new();
    // Bevy Plugins
    // Set default_nearest to prevent blurry sprits
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    // My Plugins
    app.add_plugins((PlayerPlugin, CameraPlugin));
    app.run();
}
