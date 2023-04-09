use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub mod actions;
pub mod cooldown;
pub mod mouse;

pub fn input_plugin(app: &mut App) {
    app.add_plugin(bevy_framepace::FramepacePlugin)
        .fn_plugin(mouse::mouse_plugin)
        .fn_plugin(actions::actions_plugin);
}
