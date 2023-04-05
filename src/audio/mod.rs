use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use seldom_fn_plugin::FnPluginExt;

pub mod sfx;

pub fn audio_plugin(app: &mut App) {
    app.add_plugin(AudioPlugin).fn_plugin(sfx::sfx_plugin);
}
