use std::fmt::format;

use crate::{assets::fonts::FontAssets, level::level::ReachedLevel, state::AppState};
use bevy::prelude::*;

use super::RootUiNode;

pub(super) fn game_over_ui_plugin(app: &mut App) {
    app.add_system(setup_ui.in_schedule(OnEnter(AppState::GameOver)));
}

fn setup_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    root: Res<RootUiNode>,
    reached: Res<ReachedLevel>,
) {
    commands.entity(root.0).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "GAME OVER",
            TextStyle {
                font: font_assets.menu.clone(),
                font_size: 60.0,
                color: Color::ANTIQUE_WHITE,
            },
        ));
    });

    commands.entity(root.0).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("{}", reached.0),
            TextStyle {
                font: font_assets.menu.clone(),
                font_size: 150.0,
                color: Color::ANTIQUE_WHITE,
            },
        ));
    });
}
