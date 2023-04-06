use bevy::prelude::*;

use crate::{state::GameState, AppState};

use self::echolocation::{echolocate, test_intersections};

pub mod echolocation;

pub fn echo_plugin(app: &mut App) {
    app.add_systems(
        (test_intersections, echolocate).in_set(OnUpdate(AppState::Game)), //.distributive_run_if(in_state(GameState::Running)),
    );
}
