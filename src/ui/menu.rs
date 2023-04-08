use crate::state::AppState;
use bevy::prelude::*;

use super::{
    button::{UiButton, UiButtonAction},
    RootUiNode,
};

pub(super) fn setup_ui(mut cmd: Commands, root: Res<RootUiNode>) {
    cmd.entity(root.0).with_children(|parent| {
        parent.spawn(UiButton {
            action: UiButtonAction::ChangeState(AppState::Game),
            primary: true,
            text: "PLAY".into(),
        });
    });
}
