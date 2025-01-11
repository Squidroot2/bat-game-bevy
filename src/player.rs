use bevy::prelude::*;

use crate::{
    animation::{AnimationTimer, Direction},
    constants::WINDOW_BOTTOM,
    game::Reset,
    game_over::{GameOver, GameOverReason},
    input_translation::{DirectionalInput, GameInput},
    physics::{Friction, Gravity, Velocity, WrappingMovement},
};
/// Asset path
const PLAYER_SPRITE_PATH: &str = "sprites/nf_batFlightStrip.png";
const PLAYER_SPRITE_SIZE: UVec2 = UVec2::splat(64);
const PLAYER_SPRITE_GRID: UVec2 = UVec2 { x: 8, y: 1 };

//Physics
const HORIZONTAL_ACCELERATION: f32 = 750.0;
const MAX_HORIZONTAL_SPEED: f32 = 1000.0;
const FLAP_VERTICAL_STRENGTH: f32 = 500.0;
const FLAP_HORIZONTAL_STRENGTH: f32 = 400.0;

#[derive(Event, Default)]
pub struct PlayerFlapped;

#[derive(Event, Default)]
pub struct PlayerScreetched;

#[derive(Component)]
#[require(
    Sprite,
    Velocity,
    AnimationTimer(player_animation_timer),
    WrappingMovement,
    Friction,
    Gravity,
    Direction
)]
pub struct Player;
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load(PLAYER_SPRITE_PATH);
    let atlas_layout = TextureAtlasLayout::from_grid(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_GRID.x, PLAYER_SPRITE_GRID.y, None, None);
    let atlas_layout_handle = texture_atlas_layouts.add(atlas_layout);
    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: atlas_layout_handle,
                index: 0,
            },
        ),
    ));
}

fn player_animation_timer() -> AnimationTimer {
    let frames = PLAYER_SPRITE_GRID.element_product();
    const ANIMATION_SECS: f32 = 0.2;
    AnimationTimer::new(frames, ANIMATION_SECS)
}

/// Resets a player to their original position, velocity, and animation frame
pub fn reset_player(
    mut reader: EventReader<Reset>,
    query: Single<(&mut Transform, &mut Sprite, &mut AnimationTimer, &mut Velocity), With<Player>>,
) {
    if !reader.is_empty() {
        let (mut transform, mut sprite, mut animation_timer, mut velocity) = query.into_inner();
        transform.translation = Vec3::ZERO;
        // Intentionally not flipping x sprite based on direction
        match sprite.texture_atlas.as_mut() {
            Some(atlas) => atlas.index = 0,
            None => error!("Player sprite missing texture atlas"),
        }
        animation_timer.stop();
        **velocity = Vec2::ZERO;

        reader.clear();
    }
}

pub fn check_player_crashed(mut writer: EventWriter<GameOver>, query: Single<&Transform, With<Player>>) {
    let transform = query.into_inner();
    if transform.translation.y < WINDOW_BOTTOM {
        writer.send(GameOver::new(GameOverReason::Crashed));
    }
}

//TODO determine if "Direction" is needless abstraction (It probably is tbh)
pub fn handle_input(
    mut reader: EventReader<GameInput>,
    mut screetch_writer: EventWriter<PlayerScreetched>,
    mut flap_writer: EventWriter<PlayerFlapped>,
    direction_input: Res<DirectionalInput>,
    time: Res<Time>,
    query: Single<(&mut Velocity, &mut AnimationTimer, &mut Direction), With<Player>>,
) {
    let (mut velocity, mut animation_timer, mut sprite_direction) = query.into_inner();
    let direction = direction_input.get_normalized();
    if direction < 0.0 {
        *sprite_direction = Direction::Backward;
    } else if direction > 0.0 {
        *sprite_direction = Direction::Forward;
    }

    //TODO flip sprite depending on direction faced
    velocity.x += direction * HORIZONTAL_ACCELERATION * time.delta_secs();
    velocity.x = velocity.x.clamp(-MAX_HORIZONTAL_SPEED, MAX_HORIZONTAL_SPEED);
    for input in reader.read() {
        match input {
            GameInput::Flap => {
                velocity.y += FLAP_VERTICAL_STRENGTH;
                velocity.x += direction * FLAP_HORIZONTAL_STRENGTH;
                animation_timer.start();
                flap_writer.send_default();
            }
            GameInput::Screetch => {
                screetch_writer.send_default();
            }
            _ => continue,
        }
    }
}
