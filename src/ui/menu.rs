use crate::{assets::fonts::FontAssets, palette::COL_TEXT, state::AppState, GAME_NAME};
use bevy::prelude::*;

use super::{
    button::{UiButton, UiButtonAction},
    RootUiNode,
};

pub(super) fn setup_ui(mut cmd: Commands, root: Res<RootUiNode>, font_assets: Res<FontAssets>) {
    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                GAME_NAME,
                TextStyle {
                    font: font_assets.menu.clone(),
                    font_size: 100.0,
                    color: COL_TEXT,
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Percent(5.)),
                ..default()
            },
            ..default()
        });

        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Game),
            primary: true,
            text: "play".into(),
            margin: None,
        });

        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Tutorial),
            primary: false,
            text: "tutorial".into(),
            margin: None,
        });

        if cfg!(not(target_arch = "wasm32")) {
            parent.spawn(UiButton {
                action: UiButtonAction::ChangeState(AppState::Quit),
                primary: false,
                text: "quit".into(),
                margin: Some(UiRect::top(Val::Px(40.))),
            });
        }
    });
}
