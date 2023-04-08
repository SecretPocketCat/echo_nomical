use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/circle.png")]
    pub circle: Handle<Image>,

    #[asset(path = "textures/circle_outline.png")]
    pub circle_outline: Handle<Image>,
}
