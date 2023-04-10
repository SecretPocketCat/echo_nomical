use bevy::prelude::*;

use self::camera::update_app_size;

pub mod camera;
pub mod palette;

pub fn render_plugin(app: &mut App) {
    app.add_startup_system(camera::setup_camera)
        .add_system(update_app_size);
}
