use bevy::prelude::*;

use crate::{
    constants::WINDOW_DIMENSIONS,
    input::{DirectionalInput, UserInput},
    GameState,
};
/// Asset path
const PLAYER_SPRITE_PATH: &str = "sprites/nf_batFlightStrip.png";
const PLAYER_SPRITE_SIZE: UVec2 = UVec2::splat(64);
const PLAYER_SPRITE_GRID: UVec2 = UVec2 { x: 8, y: 1 };

//Physics
const GRAVITY: f32 = 1500.0;
const HORIZONTAL_ACCELERATION: f32 = 750.0;
const MAX_HORIZONTAL_SPEED: f32 = 1000.0;
const FLAP_VERTICAL_STRENGTH: f32 = 500.0;
const FLAP_HORIZONTAL_STRENGTH: f32 = 400.0;
const FRICTION: f32 = 0.05;

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
#[require(Sprite, Velocity)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            Update,
            (
                move_player,
                handle_input.before(move_player),
                add_gravity.before(move_player),
                add_friction.after(move_player),
                wrap_position.after(move_player),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
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

fn wrap_position(query: Single<&mut Transform, With<Player>>) {
    let mut transform = query.into_inner();

    //TODO division is expensive, should probably save as const
    if transform.translation.x < (-WINDOW_DIMENSIONS.x / 2.0) {
        println!("Wrapped Left!");
        transform.translation.x += WINDOW_DIMENSIONS.x
    } else if transform.translation.x > (WINDOW_DIMENSIONS.x / 2.0) {
        println!("Wrapped Right!");
        transform.translation.x -= WINDOW_DIMENSIONS.x
    }
}

fn add_friction(time: Res<Time>, query: Single<&mut Velocity, With<Player>>) {
    let mut velocity = query.into_inner();
    **velocity *= 1.0 - (FRICTION * time.delta_secs());
}

fn move_player(time: Res<Time>, query: Single<(&Velocity, &mut Transform), With<Player>>) {
    let (vel, mut transform) = query.into_inner();
    let movement = vel.extend(0.0) * time.delta_secs();
    transform.translation += movement;
}

fn handle_input(
    mut reader: EventReader<UserInput>,
    direction_input: Res<DirectionalInput>,
    time: Res<Time>,
    query: Single<&mut Velocity, With<Player>>,
) {
    let mut velocity = query.into_inner();
    let direction = direction_input.get_normalized();
    //TODO flip sprite depending on direction faced
    velocity.x += direction * HORIZONTAL_ACCELERATION * time.delta_secs();
    velocity.x = velocity.x.clamp(-MAX_HORIZONTAL_SPEED, MAX_HORIZONTAL_SPEED);
    for input in reader.read() {
        if matches!(input, UserInput::Flap) {
            velocity.y += FLAP_VERTICAL_STRENGTH;
            velocity.x += direction * FLAP_HORIZONTAL_STRENGTH;
        }
    }
}
fn add_gravity(time: Res<Time>, query: Single<&mut Velocity, With<Player>>) {
    let mut velocity = query.into_inner();
    let velocity_change = Vec2::new(0.0, -GRAVITY) * time.delta_secs();
    **velocity += velocity_change;
}
