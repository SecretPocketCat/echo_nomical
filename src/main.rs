// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use seldom_fn_plugin::FnPluginExt;
use echo_nomical::LevelSize;
use echo_nomical::GAME_NAME;

fn main() {
    let lvl_size = LevelSize(Vec2::new(800., 600.));

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
