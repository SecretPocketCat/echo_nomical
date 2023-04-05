use std::{ops::RangeInclusive, time::Duration};

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

use crate::{time::time::after_delay, AppState};

pub mod audio;
pub mod fonts;
mod splash;
pub mod textures;
mod window_icon;

use self::{splash::print_progress, window_icon::set_window_icon};

pub fn assets_plugin(app: &mut App) {
    app.add_startup_system(set_window_icon)
        // .add_loading_state(LoadingState::new(AppState::Splash).continue_to_state(AppState::Loading))
        // .add_collection_to_loading_state::<_, splash::SplashScreenTextureAssets>(AppState::Splash)
        .add_loading_state(LoadingState::new(AppState::Loading))
        .add_collection_to_loading_state::<_, fonts::FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, audio::SfxAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, audio::AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, textures::TextureAssets>(AppState::Loading)
        .add_startup_system(audio::setup_sfx_assets)
        // .add_plugin(ProgressPlugin::new(AppState::Splash))
        .add_plugin(ProgressPlugin::new(AppState::Loading).continue_to(AppState::Game))
        .add_system(
            print_progress
                .in_set(OnUpdate(AppState::Loading))
                .run_if(after_delay(Duration::from_secs(3))),
        );
}

fn add_dynamic_assets(
    dynamic_assets: &mut DynamicAssets,
    key: &str,
    file_prefix: &str,
    file_ext: &str,
    range: RangeInclusive<usize>,
) {
    dynamic_assets.register_asset(
        key,
        Box::new(StandardDynamicAsset::Files {
            paths: range
                .map(|i| format!("{file_prefix}{i}.{file_ext}"))
                .collect(),
        }),
    );
}
