use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryCamera;

pub(super) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));
}
