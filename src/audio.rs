use bevy::prelude::*;

use crate::{
    game::EnemyEaten,
    player::{PlayerFlapped, PlayerScreetched},
};

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub struct SoundEffectSystem;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoundHandler>();
        app.add_systems(Startup, load_sounds);
        app.add_systems(
            Update,
            (
                play_flap.run_if(on_event::<PlayerFlapped>),
                play_screetch.run_if(on_event::<PlayerScreetched>),
                play_munch.run_if(on_event::<EnemyEaten>),
            )
                .in_set(SoundEffectSystem),
        );
    }
}
/// Holds on to Strong Handles for Sound Effects
#[derive(Resource, Default)]
pub struct SoundHandler {
    flap: Handle<AudioSource>,
    munch: Handle<AudioSource>,
    screetch: Handle<AudioSource>,
}

impl SoundHandler {
    const FLAP_SOUND_PATH: &str = "sounds/flap.ogg";
    const MUNCH_SOUND_PATH: &str = "sounds/munch.ogg";
    const SQUEAK_SOUND_PATH: &str = "sounds/squeak.ogg";

    fn load(&mut self, asset_server: Res<AssetServer>) {
        *self = Self {
            flap: asset_server.load(Self::FLAP_SOUND_PATH),
            munch: asset_server.load(Self::MUNCH_SOUND_PATH),
            screetch: asset_server.load(Self::SQUEAK_SOUND_PATH),
        }
    }
}

pub fn load_sounds(asset_server: Res<AssetServer>, mut sound_handler: ResMut<SoundHandler>) {
    sound_handler.load(asset_server);
}
pub fn play_flap(mut commands: Commands, sound_handler: Res<SoundHandler>) {
    commands.spawn((AudioPlayer::new(sound_handler.flap.clone()), PlaybackSettings::DESPAWN));
}
pub fn play_screetch(mut commands: Commands, sound_handler: Res<SoundHandler>) {
    commands.spawn((AudioPlayer::new(sound_handler.screetch.clone()), PlaybackSettings::DESPAWN));
}
pub fn play_munch(mut commands: Commands, sound_handler: Res<SoundHandler>) {
    commands.spawn((AudioPlayer::new(sound_handler.munch.clone()), PlaybackSettings::DESPAWN));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn files_present() {
        assert_asset_present(SoundHandler::FLAP_SOUND_PATH);
        assert_asset_present(SoundHandler::SQUEAK_SOUND_PATH);
        assert_asset_present(SoundHandler::MUNCH_SOUND_PATH);
    }

    fn assert_asset_present(asset_path: &str) {
        let mut path = PathBuf::from("assets");
        path.push(asset_path);
        assert!(path.exists());
    }
}
