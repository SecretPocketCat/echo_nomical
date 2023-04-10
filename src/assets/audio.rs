use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use super::add_dynamic_assets;

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    // todo: fix echo
    #[asset(key = "echo", collection(typed))]
    pub echo: Vec<Handle<AudioSource>>,

    #[asset(key = "player_death", collection(typed))]
    pub player_death: Vec<Handle<AudioSource>>,

    #[asset(key = "enemy_alert", collection(typed))]
    pub enemy_alert: Vec<Handle<AudioSource>>,

    #[asset(key = "enemy_death", collection(typed))]
    pub enemy_death: Vec<Handle<AudioSource>>,

    #[asset(path = "audio/sfx/level_cleared.ogg")]
    pub level_cleared: Handle<AudioSource>,

    #[asset(path = "audio/sfx/drone.ogg")]
    pub drone: Handle<AudioSource>,
}

pub(super) fn setup_sfx_assets(mut dynamic_assets: ResMut<DynamicAssets>) {
    for key in ["echo", "player_death", "enemy_alert", "enemy_death"] {
        add_dynamic_assets(
            &mut dynamic_assets,
            key,
            &format!("audio/sfx/{}", key),
            "ogg",
            1..=2,
        );
    }
}
