use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub mod menu;

pub fn ui_plugin(app: &mut App) {
    app.fn_plugin(menu::menu_plugin);
}
