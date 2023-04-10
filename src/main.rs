// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use seldom_fn_plugin::FnPluginExt;
use sonarscape::palette::COL_BG;
use sonarscape::AppSize;
use sonarscape::GAME_NAME;

fn main() {
    let lvl_size = AppSize(Vec2::new(1000., 800.));

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(COL_BG))
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
        .fn_plugin(sonarscape::game_plugin)
        .run();
}
