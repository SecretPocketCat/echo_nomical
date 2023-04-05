use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub mod actions;
pub mod mouse;

pub fn input_plugin(app: &mut App) {
    app.fn_plugin(mouse::mouse_plugin)
        .fn_plugin(actions::actions_plugin);
}
