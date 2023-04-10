use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::{palette::COL_BG, AppSize};

#[derive(Component)]
pub struct PrimaryCamera;

pub(super) fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(COL_BG),
                ..default()
            },
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        PrimaryCamera,
        BloomSettings::default(),
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
