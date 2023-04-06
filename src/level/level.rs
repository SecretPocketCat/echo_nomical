use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource, Deref)]
pub struct LevelSize(pub Vec2);

pub(super) fn setup_test_lvl(mut cmd: Commands) {
    for polyline in [
        // walls
        vec![
            (-350., 350.),
            (400., 360.),
            (380., 30.),
            (480., -330.),
            (420., -360.),
            (100., -310.),
            (-380., -280.),
            (-425., 280.),
            (-350., 350.),
        ],
        vec![(-250., 200.), (-200., 230.), (-170., 150.), (-250., 200.)],
        // (0., 375., 500., 40.),
        // (0., -375., 500., 40.),
        // (500., 0., 40., 400.),
        // (-500., 0., 40., 400.),
        // obstacles
        // (300., 220., 40., 60.),
        // (-180., -230., 50., 50.),
        // (-120., 120., 50., 50.),
    ] {
        cmd.spawn(Collider::polyline(
            polyline.iter().map(|(x, y)| Vec2::new(*x, *y)).collect(),
            None,
        ));
    }
}
