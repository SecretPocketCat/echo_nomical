use bevy::prelude::*;

use self::player::{exit_reached, move_player, spawn_player};
use crate::{state::UnpausedGame, AppState};

pub mod player;

pub fn player_plugin(app: &mut App) {
    app.add_system(spawn_player.in_set(OnUpdate(AppState::Game)))
        .add_systems((move_player, exit_reached).in_set(UnpausedGame));
}
