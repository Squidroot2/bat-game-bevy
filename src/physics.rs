use std::time::Duration;

use bevy::prelude::*;

use crate::constants::WINDOW_DIMENSIONS;

#[derive(Component, Default)]
pub struct WrappingMovement;

#[derive(Component)]
pub struct Gravity {
    acceleration: f32,
}

impl Gravity {
    const DEFAULT_VALUE: f32 = 1500.0;
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            acceleration: Self::DEFAULT_VALUE,
        }
    }
}

#[derive(Component)]
pub struct Friction {
    factor: f32,
}

impl Friction {
    const DEFAULT_FACTOR: f32 = 0.05;
}

impl Default for Friction {
    fn default() -> Self {
        Self {
            factor: Self::DEFAULT_FACTOR,
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    fn add_friction(&mut self, friction: &Friction, time_delta: Duration) {
        **self *= 1.0 - (friction.factor * time_delta.as_secs_f32());
    }

    fn add_gravity(&mut self, gravity: &Gravity, time_delta: Duration) {
        **self += Vec2::new(0.0, -gravity.acceleration * time_delta.as_secs_f32())
    }
}

pub fn wrap_position(mut query: Query<&mut Transform, With<WrappingMovement>>) {
    const HORIZONTAL_LIMIT: f32 = WINDOW_DIMENSIONS.x / 2.0;
    for mut transform in query.iter_mut() {
        if transform.translation.x < -HORIZONTAL_LIMIT {
            transform.translation.x += WINDOW_DIMENSIONS.x;
        } else if transform.translation.x > HORIZONTAL_LIMIT {
            transform.translation.x -= WINDOW_DIMENSIONS.x;
        }
    }
}

pub fn add_friction(time: Res<Time>, mut query: Query<(&mut Velocity, &Friction)>) {
    for (mut velocity, friction) in query.iter_mut() {
        velocity.add_friction(friction, time.delta());
    }
}

pub fn move_with_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let movement = velocity.extend(0.0) * time.delta_secs();
        transform.translation += movement;
    }
}

pub fn add_gravity(time: Res<Time>, mut query: Query<(&mut Velocity, &Gravity)>) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.add_gravity(gravity, time.delta());
    }
}
