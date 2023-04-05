use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Deref)]
pub struct LevelSize(pub Vec2);

pub(super) fn setup_test_lvl(mut cmd: Commands) {
    cmd.spawn(SpatialBundle::from(Transform::from_xyz(
        -200.0, -100.0, 0.0,
    )))
    .insert(Collider::cuboid(500.0, 50.0));
}
