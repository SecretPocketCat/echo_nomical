use bevy::prelude::*;

use crate::{assets::textures::TextureAssets, ui::RootUiNode};

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
