use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryCamera;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            ..default()
        },
        PrimaryCamera,
    ));
}
