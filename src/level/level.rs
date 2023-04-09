use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::*;

use crate::{
    echolocation::echolocation::EcholocationHitColor, enemy::SpawnEnemyEv,
    player::player::PlayerEv, render::camera::PrimaryCamera, AppSize,
};

use super::mapgen::{gen_map, TileType};

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

#[derive(Resource, Default)]
pub struct ReachedLevel(pub usize);

pub(super) fn setup_test_lvl(
    mut cmd: Commands,
    mut ev_w: EventWriter<SpawnEnemyEv>,
    bounds: Res<AppSize>,
) {
    // let bounds = &*app_size;
    let map = gen_map();
    for y in 0..map.height {
        for x in 0..map.width {
            if map.xy(x, y) == &TileType::Wall {
                let x = 40 * x - (bounds.x as i32) / 2;
                let y = 40 * y - (bounds.y as i32) / 2;
                cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
                    x as f32, y as f32, 0.,
                )))
                .insert(Collider::cuboid(20., 20.));
            }
        }
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
    for (x, y) in [(-200., -100.), (360., -250.), (0., 200.)].iter() {
        ev_w.send(SpawnEnemyEv(Vec2::new(*x, *y)));
    }
}

pub(super) fn update_score(mut ev_r: EventReader<PlayerEv>, mut reached: ResMut<ReachedLevel>) {
    for ev in ev_r.iter() {
        if let PlayerEv::ClearedLevel = ev {
            reached.0 += 1;
        }
    }
}
