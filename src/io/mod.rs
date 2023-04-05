use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub mod save;

pub fn io_plugin(app: &mut App) {
    app.fn_plugin(save::save_plugin);
}
