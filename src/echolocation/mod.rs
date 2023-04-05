use bevy::prelude::*;

use crate::{state::GameState, AppState};

use self::echolocation::test_intersections;

pub mod echolocation;

pub fn echo_plugin(app: &mut App) {
    app.add_system(
        test_intersections
            .in_set(OnUpdate(AppState::Game))
            .run_if(in_state(GameState::Running)),
    );
}
