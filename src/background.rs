use bevy::prelude::*;

use crate::constants::WINDOW_DIMENSIONS;

const BACKGROUND_IMAGE_PATH: &str = "sprites/background.png";

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load(BACKGROUND_IMAGE_PATH);
    let mut sprite = Sprite::from_image(texture);
    sprite.custom_size = Some(WINDOW_DIMENSIONS);
    let transform = Transform::from_xyz(0.0, 0.0, -1.0);
    commands.spawn((sprite, transform));
}
