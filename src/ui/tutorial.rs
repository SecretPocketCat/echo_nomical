use crate::{
    assets::{fonts::FontAssets, textures::TextureAssets},
    palette::{COL_BG, COL_TEXT},
    state::AppState,
    GAME_NAME,
};
use bevy::{prelude::*, text::BreakLineOn};

use super::{
    button::{UiButton, UiButtonAction},
    RootUiNode, UiDisabled,
};

pub(super) fn setup_ui(
    mut cmd: Commands,
    root: Res<RootUiNode>,
    font_assets: Res<FontAssets>,
    tex: Res<TextureAssets>,
) {
    cmd.entity(root.0).with_children(|parent| {
        for txt in [
            "Welcome to the depths.",
            "Your goal is to reach as deep as possible.",
            "Move with the arrow keys or WASD or use a controller.",
        ] {
            spawn_text(parent, &font_assets, txt);
        }

        parent.spawn(ImageBundle {
            image: UiImage::new(tex.controls_arrows.clone()),
            transform: Transform::from_scale(Vec2::splat(0.6).extend(1.)),
            ..default()
        });

        for txt in [
            "Echolocate with space to explore your surroundings,",
            "but beware of foes that might be lurking in the dark.",
        ] {
            spawn_text(parent, &font_assets, txt);
        }

        parent
            .spawn(ImageBundle {
                image: UiImage::new(tex.button.clone()),
                style: Style {
                    padding: UiRect {
                        top: Val::Px(10.),
                        bottom: Val::Px(10.),
                        left: Val::Px(30.),
                        right: Val::Px(30.),
                    },
                    margin: UiRect {
                        top: Val::Px(20.),
                        bottom: Val::Px(35.),
                        ..default()
                    },
                    ..default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "space",
                    TextStyle {
                        font: font_assets.menu.clone(),
                        font_size: 60.,
                        color: COL_BG,
                    },
                ));
            });

        for txt in ["Find the portal to reach the next level and delve deeper."] {
            spawn_text(parent, &font_assets, txt);
        }

        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Game),
            primary: true,
            text: "play".into(),
            margin: None,
        });
    });
}

fn spawn_text(parent: &mut ChildBuilder, font_assets: &FontAssets, txt: &str) {
    parent.spawn(TextBundle {
        text: Text::from_section(
            txt,
            TextStyle {
                font: font_assets.menu.clone(),
                font_size: 22.0,
                color: COL_TEXT,
            },
        )
        .with_alignment(TextAlignment::Center),
        style: Style {
            margin: UiRect::bottom(Val::Px(15.)),
            ..default()
        },
        ..default()
    });
}
