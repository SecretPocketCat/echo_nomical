use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::*;

use crate::{
    echolocation::echolocation::EcholocationHitColor,
    enemy::{EnemyType, SpawnEnemyEv},
    player::player::PlayerEv,
};

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

#[derive(Resource, Default)]
pub struct ReachedLevel(pub usize);

pub(super) fn setup_test_lvl(mut cmd: Commands, mut ev_w: EventWriter<SpawnEnemyEv>) {
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
    .insert(Collider::round_cuboid(25., 25., 0.25))
    .insert(Sensor)
    .insert(LevelExit)
    .insert(EcholocationHitColor(Color::GOLD))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(ActiveCollisionTypes::all());

    // enemies
    for (x, y, enemy_type) in [
        (-200., -100., EnemyType::Bouncy),
        (360., -250., EnemyType::Spiky),
        (0., 200., EnemyType::Dasher),
    ]
    .iter()
    {
        ev_w.send(SpawnEnemyEv {
            position: Vec2::new(*x, *y),
            enemy_type: *enemy_type,
        });
    }
}

pub(super) fn update_score(mut ev_r: EventReader<PlayerEv>, mut reached: ResMut<ReachedLevel>) {
    for ev in ev_r.iter() {
        if let PlayerEv::ClearedLevel = ev {
            reached.0 += 1;
        }
    }
}
