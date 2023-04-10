use bevy::prelude::*;
use bevy_tweening::EaseFunction;
use interpolation::Ease;

use crate::{
    animation::{get_relative_scale_anim, get_scale_anim, TweenDoneAction},
    assets::textures::TextureAssets,
    state::AppState,
    ui::RootUiNode,
};

pub(super) fn show_splash(mut cmd: Commands, root: Res<RootUiNode>, tex: Res<TextureAssets>) {
    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(ImageBundle {
            image: UiImage::new(tex.bevy.clone()),
            style: Style {
                max_size: Size::all(Val::Percent(100.)),
                ..default()
            },
            ..default()
        });
    });
}
