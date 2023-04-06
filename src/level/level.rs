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
        ],
        // obstacles
        vec![(-250., 200.), (-200., 230.), (-170., 150.)],
        vec![(250., 200.), (400., 230.), (380., 90.)],
        vec![(300., 280.), (220., 300.), (-200., -210.), (100., -180.)],
    ] {
        let mut vertices: Vec<_> = polyline.iter().map(|(x, y)| Vec2::new(*x, *y)).collect();
        vertices.push(polyline[0].into());
        cmd.spawn(Collider::polyline(vertices, None));
    }
}
