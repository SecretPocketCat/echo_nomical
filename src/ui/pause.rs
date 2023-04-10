use crate::{
    animation::{get_relative_text_color_anim, TweenDoneAction},
    assets::fonts::FontAssets,
    palette::COL_TEXT,
    state::AppState,
};
use bevy::prelude::*;

use super::{
    button::{UiButton, UiButtonAction},
    RootUiNode,
};

pub(super) fn setup_ui(mut cmd: Commands, font_assets: Res<FontAssets>, root: Res<RootUiNode>) {
    cmd.entity(root.0).with_children(|parent| {
        parent
            .spawn(TextBundle {
                text: Text::from_section(
                    "paused",
                    TextStyle {
                        font: font_assets.menu.clone(),
                        font_size: 60.0,
                        color: Color::NONE,
                    },
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Percent(5.)),
                    ..default()
                },
                ..default()
            })
            .insert(get_relative_text_color_anim(
                COL_TEXT,
                400,
                TweenDoneAction::None,
            ));

        parent.spawn(UiButton {
            action: UiButtonAction::Unpause,
            primary: true,
            text: "continue".into(),
            margin: None,
        });

        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Game),
            primary: false,
            text: "restart".into(),
            margin: None,
        });

        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Menu),
            primary: false,
            text: "menu".into(),
            margin: None,
        });
    });
}
