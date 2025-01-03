use bevy::{prelude::*, render::camera::ScalingMode};

use crate::constants::WINDOW_DIMENSIONS;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::Fixed {
            width: WINDOW_DIMENSIONS.x,
            height: WINDOW_DIMENSIONS.y,
        },
        ..OrthographicProjection::default_2d()
    });
    commands.spawn((Camera2d, projection));
}
