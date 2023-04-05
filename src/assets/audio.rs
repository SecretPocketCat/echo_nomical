use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use super::add_dynamic_assets;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    #[asset(key = "click", collection(typed))]
    pub click: Vec<Handle<AudioSource>>,
}

pub(super) fn setup_sfx_assets(mut dynamic_assets: ResMut<DynamicAssets>) {
    add_dynamic_assets(
        &mut dynamic_assets,
        "click",
        "audio/sfx/ui/click",
        "ogg",
        1..=13,
    );
}
