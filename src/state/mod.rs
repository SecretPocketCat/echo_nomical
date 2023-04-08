use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;
pub use state::AppState;
pub use state::GameState;

mod pause;
mod reset;
mod state;

pub use reset::{FadeReset, PersistReset};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct UnpausedGame;

pub fn state_plugin(app: &mut App) {
    app.add_state::<AppState>()
        .add_state::<GameState>()
        .configure_set(
            UnpausedGame.run_if(in_state(AppState::Game).and_then(in_state(GameState::Running))),
        )
        .fn_plugin(pause::pause_plugin)
        .fn_plugin(reset::reset_plugin);
}
