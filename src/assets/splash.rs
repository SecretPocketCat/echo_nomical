use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::ProgressCounter;

#[derive(AssetCollection, Resource)]
pub struct SplashScreenTextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
}

pub(super) fn print_progress(progress: Res<ProgressCounter>) {
    let progress = progress.progress();

    if progress.total > 0 && progress.done == progress.total {}
}
