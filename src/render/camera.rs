use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::AppSize;

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

pub(super) fn update_app_size(
    mut ev_r: EventReader<WindowResized>,
    win_q: Query<(), With<PrimaryWindow>>,
    mut size: ResMut<AppSize>,
) {
    if let Some(ev) = ev_r.iter().find(|ev| win_q.contains(ev.window)) {
        size.0 = Vec2::new(ev.width, ev.height);
    }
}
