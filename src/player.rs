use bevy::prelude::*;

const PLAYER_SPRITE_PATH: &str = "sprites/nf_batFlightStrip.png";
const PLAYER_SPRITE_SIZE: UVec2 = UVec2::splat(64);
const PLAYER_SPRITE_GRID: UVec2 = UVec2 { x: 8, y: 1 };

#[derive(Component)]
#[require(Sprite)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    println!("Spawned Player");

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
