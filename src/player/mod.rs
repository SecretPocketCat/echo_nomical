use bevy::prelude::*;

use self::player::{exit_reached, move_player, player_hit, spawn_player, PlayerEv};
use crate::{state::UnpausedGame, AppState};

pub mod player;

pub fn player_plugin(app: &mut App) {
    app.add_event::<PlayerEv>()
        .add_system(spawn_player.in_set(OnUpdate(AppState::Game)))
        .add_systems((move_player, exit_reached, player_hit).in_set(UnpausedGame));
}
