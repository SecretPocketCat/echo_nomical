use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::*;

#[derive(Resource, Deref)]
pub struct LevelSize(pub Vec2);

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

pub(super) fn setup_test_lvl(mut cmd: Commands) {
    let mut rng = thread_rng();

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
        let mut vertices: Vec<_> = polyline
            .iter()
            .map(|(x, y)| {
                Vec2::new(
                    *x + rng.gen_range(-50.0..50.),
                    *y + rng.gen_range(-50.0..50.),
                )
            })
            .collect();
        vertices.push(vertices[0]);
        cmd.spawn(Collider::polyline(vertices, None));
    }

    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
        330., 20., 0.,
    )))
    .insert(LevelEntry);

    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
        -325., 260., 0.,
    )))
    .insert(Collider::cuboid(25., 25.))
    .insert(Sensor)
    .insert(LevelExit)
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(ActiveCollisionTypes::all());
}
