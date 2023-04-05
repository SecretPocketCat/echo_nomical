use bevy::prelude::*;

use crate::state::AppState;

use self::level::setup_test_lvl;

pub mod level;

pub fn lvl_plugin(app: &mut App) {
    app.add_system(setup_test_lvl.in_schedule(OnEnter(AppState::Game)));
}
