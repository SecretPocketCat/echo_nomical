use crate::{
    assets::fonts::FontAssets, level::level::ReachedLevel, palette::COL_TEXT, state::AppState,
};
use bevy::prelude::*;

use super::{
    button::{UiButton, UiButtonAction},
    RootUiNode,
};

pub(super) fn setup_ui(
    mut cmd: Commands,
    font_assets: Res<FontAssets>,
    root: Res<RootUiNode>,
    reached: Res<ReachedLevel>,
) {
    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "GAME OVER",
            TextStyle {
                font: font_assets.menu.clone(),
                font_size: 60.0,
                color: COL_TEXT,
            },
        ));
    });

    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                format!("{}", reached.0),
                TextStyle {
                    font: font_assets.menu.clone(),
                    font_size: 150.0,
                    color: COL_TEXT,
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Percent(10.)),
                ..default()
            },
            ..default()
        });
    });

    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Game),
            primary: true,
            text: "PLAY AGAIN".into(),
            margin: None,
        });
    });

    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Menu),
            primary: false,
            text: "MENU".into(),
            margin: None,
        });
    });
}
