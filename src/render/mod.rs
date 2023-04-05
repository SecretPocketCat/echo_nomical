use bevy::prelude::*;

pub mod camera;

pub fn render_plugin(app: &mut App) {
    app.add_startup_system(camera::setup_camera);
}
