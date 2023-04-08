// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use echo_nomical::AppSize;
use echo_nomical::GAME_NAME;
use seldom_fn_plugin::FnPluginExt;

fn main() {
    let lvl_size = AppSize(Vec2::new(1000., 800.));

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: GAME_NAME.to_string(), // ToDo
                resolution: (lvl_size.0).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(lvl_size)
        .fn_plugin(echo_nomical::game_plugin)
        .run();
}
