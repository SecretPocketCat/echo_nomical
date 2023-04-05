use bevy::prelude::*;

use self::player::{move_player, spawn_player};
use crate::{state::GameState, AppState};

pub mod player;

pub fn player_plugin(app: &mut App) {
    app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_system(
            move_player
                .in_set(OnUpdate(AppState::Game))
                .run_if(in_state(GameState::Running)),
        );
}
