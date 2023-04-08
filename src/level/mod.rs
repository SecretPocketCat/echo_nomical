use bevy::prelude::*;

use crate::state::AppState;

use self::level::{setup_test_lvl, update_score, PlayerEvent, ReachedLevel};

pub mod level;

pub fn lvl_plugin(app: &mut App) {
    app.init_resource::<ReachedLevel>()
        .add_event::<PlayerEvent>()
        .add_system(setup_test_lvl.in_schedule(OnEnter(AppState::Game)))
        .add_system(update_score);
}
