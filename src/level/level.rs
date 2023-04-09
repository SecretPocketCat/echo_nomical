use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::*;

use crate::{
    agent::agent::Bouncable,
    echolocation::echolocation::EcholocationHitColor,
    enemy::{EnemyType, SpawnEnemyEv},
    player::player::PlayerEv,
    render::camera::PrimaryCamera,
    AppSize,
};

use super::mapgen::{gen_map, TileType};

// use super::mapgen::{gen_map, TileType};

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

#[derive(Resource, Default)]
pub struct ReachedLevel(pub usize);

#[derive(Component)]
pub struct Wall;

pub(super) fn setup_test_lvl(
    mut cmd: Commands,
    mut ev_w: EventWriter<SpawnEnemyEv>,
    bounds: Res<AppSize>,
) {
    let bounds = &*bounds;
    let map = gen_map();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.xy(x, y) == &TileType::Wall {
                let x = 40 * x - (bounds.x as i32) / 2;
                let y = 40 * y - (bounds.y as i32) / 2;
                cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
                    x as f32, y as f32, 0.,
                )))
                .insert(Collider::cuboid(20., 20.))
                .insert(Wall)
                .insert(Bouncable)
                .insert(Name::new("Wall"));
            }
        }
    }

    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
        330., 20., 0.,
    )))
    .insert(LevelEntry)
    .insert(Name::new("Entry"));

    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
        -325., 260., 0.,
    )))
    .insert(Collider::round_cuboid(25., 25., 0.25))
    .insert(Sensor)
    .insert(LevelExit)
    .insert(EcholocationHitColor(Color::GOLD))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(ActiveCollisionTypes::all())
    .insert(Name::new("Exit"));

    // enemies
    for (x, y, enemy_type) in [
        (-200., -100., EnemyType::Static),
        (360., -250., EnemyType::Static),
        (0., 200., EnemyType::FollowPing),
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
