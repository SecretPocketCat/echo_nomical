use bevy::prelude::*;

use crate::state::UnpausedGame;

use self::echolocation::{echo_hit, echolocate, EcholocationHitEv};

pub mod echolocation;

pub fn echo_plugin(app: &mut App) {
    app.add_event::<EcholocationHitEv>()
        .add_systems((echolocate, echo_hit).in_set(UnpausedGame));
}
