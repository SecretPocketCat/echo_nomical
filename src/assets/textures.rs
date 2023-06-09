use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "textures/echo_ping.png")]
    pub echo_ping: Handle<Image>,

    #[asset(path = "textures/wave.png")]
    pub wave: Handle<Image>,

    #[asset(path = "textures/spiky.png")]
    pub spiky: Handle<Image>,

    #[asset(path = "textures/charge.png")]
    pub charge: Handle<Image>,

    #[asset(path = "textures/portal.png")]
    pub portal: Handle<Image>,

    #[asset(path = "textures/button.png")]
    pub button: Handle<Image>,

    #[asset(path = "textures/controls_arrows.png")]
    pub controls_arrows: Handle<Image>,
}
