use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Deref)]
pub struct LevelSize(pub Vec2);

pub(super) fn setup_test_lvl(mut cmd: Commands) {
    for (x, y, w, h) in [
        // walls
        (0., 375., 500., 40.),
        (0., -375., 500., 40.),
        (500., 0., 40., 400.),
        (-500., 0., 40., 400.),
        // obstacles
        (300., 220., 40., 60.),
        (-180., -230., 50., 50.),
        (-120., 120., 50., 50.),
    ] {
        cmd.spawn(SpatialBundle::from(Transform::from_xyz(x, y, 0.0)))
            .insert(Collider::cuboid(w, h));
    }
}
