use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationTimer {
    frames: u32,
    timer: Timer,
    current_frame: u32,
}

/// Represent a direction that a sprite is facing
#[derive(Component, Default, Eq, PartialEq)]
pub enum Direction {
    #[default]
    Forward,
    Backward,
}

impl AnimationTimer {
    pub fn new(frames: u32, total_secs: f32) -> Self {
        let duration = Duration::from_secs_f32(total_secs / (frames as f32));
        Self {
            frames,
            timer: Timer::new(duration, TimerMode::Repeating),
            current_frame: 0,
        }
    }

    pub fn start(&mut self) {
        self.timer.reset();
        self.timer.unpause();
        self.current_frame = 0;
    }

    /// Resets and stops the animation
    pub fn stop(&mut self) {
        self.timer.reset();
        self.timer.pause();
        self.current_frame = 0;
    }

    fn get_current_frame(&mut self, tick: Duration) -> u32 {
        self.timer.tick(tick);
        self.current_frame += self.timer.times_finished_this_tick();
        if self.current_frame >= self.frames {
            self.timer.pause();
            self.current_frame = 0;
        }
        self.current_frame
    }
}

pub fn animate_sprites(mut query: Query<(&mut Sprite, &mut AnimationTimer)>, time: Res<Time>) {
    for (mut sprite, mut animation_timer) in query.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = animation_timer.get_current_frame(time.delta()) as usize;
        }
    }
}

pub fn direct_sprites(mut query: Query<(&mut Sprite, &Direction)>) {
    for (mut sprite, direction) in query.iter_mut() {
        sprite.flip_x = *direction == Direction::Backward;
    }
}
