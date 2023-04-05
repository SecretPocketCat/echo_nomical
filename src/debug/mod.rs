use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

pub fn debug_plugin(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        app.add_plugin(EditorPlugin::default());

        // app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .add_plugin(LogDiagnosticsPlugin::default());
    }
}
