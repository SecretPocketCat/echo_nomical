use bevy::prelude::*;

use crate::state::UnpausedGame;

use self::echolocation::{echolocate, test_intersections};

pub mod echolocation;

pub fn echo_plugin(app: &mut App) {
    app.add_systems((test_intersections, echolocate).in_set(UnpausedGame));
}
