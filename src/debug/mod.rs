use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub fn debug_plugin(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        app.add_plugin(EditorPlugin::default())
            .add_plugin(ShapePlugin)
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
        // .add_plugin(LogDiagnosticsPlugin::default());
    }
}
