#![feature(let_chains)]

use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

use state::AppState;

pub use tools::ecs::EntityCommandsExt;

mod agent;
mod animation;
mod assets;
mod audio;
mod debug;
mod echolocation;
mod enemy;
mod input;
mod io;
mod level;
mod physics;
mod player;
mod render;
mod state;
mod time;
mod tools;
mod ui;

#[derive(Resource, Deref)]
pub struct AppSize(pub Vec2);

pub const GAME_NAME: &str = "todo";
pub use render::palette;

pub fn game_plugin(app: &mut App) {
    app.fn_plugin(state::state_plugin)
        .fn_plugin(animation::animation_plugin)
        .fn_plugin(assets::assets_plugin)
        .fn_plugin(physics::physics_plugin)
        .fn_plugin(audio::audio_plugin)
        .fn_plugin(agent::agent_plugin)
        .fn_plugin(render::render_plugin)
        .fn_plugin(ui::ui_plugin)
        .fn_plugin(input::input_plugin)
        .fn_plugin(player::player_plugin)
        .fn_plugin(echolocation::echo_plugin)
        .fn_plugin(time::time_plugin)
        .fn_plugin(level::lvl_plugin)
        .fn_plugin(enemy::enemy_plugin)
        .fn_plugin(io::io_plugin);

    #[cfg(debug_assertions)]
    {
        app.fn_plugin(debug::debug_plugin);
    }
}
