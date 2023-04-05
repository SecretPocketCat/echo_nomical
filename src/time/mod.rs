use bevy::prelude::*;

use crate::state::GameState;

use self::time::{update_time_scale, TimeScale};

pub mod time;

pub fn time_plugin(app: &mut App) {
    app.insert_resource(TimeScale(1.))
        .add_system(update_time_scale::<0>.in_schedule(OnEnter(GameState::Paused)))
        .add_system(update_time_scale::<1>.in_schedule(OnExit(GameState::Paused)));
}
