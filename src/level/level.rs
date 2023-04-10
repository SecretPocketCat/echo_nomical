use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::mapgen::{gen_map, TileType};
use crate::{
    agent::agent::AgentRotation,
    assets::textures::TextureAssets,
    echolocation::echolocation::EcholocationHitColor,
    enemy::{EnemyType, SpawnEnemyEv},
    player::player::PlayerEv,
    AppSize,
};

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
    tex: Res<TextureAssets>,
) {
    let scaling_factor = 40.0;
    let half_sf = scaling_factor / 2.;
    let tiles = (bounds.0 / scaling_factor).as_ivec2();
    let map = gen_map(tiles.x, tiles.y);

    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.xy(x, y);
            let (x, y) = (x as f32, y as f32);
            let x = scaling_factor * x - bounds.x / 2.;
            let y = scaling_factor * y - bounds.y / 2.;
            match tile {
                &TileType::Wall => {
                    cmd
                        .spawn(TransformBundle::from_transform(Transform::from_xyz(
                            x + half_sf, y + half_sf, 0.,
                        )))
                        .insert(Collider::cuboid(half_sf, half_sf))
                        .insert(Wall)
                        .insert(Name::new("Wall"));
                },
                &TileType::Enemy(enemy_type) => ev_w.send(SpawnEnemyEv {
                    position: Vec2::new(x, y),
                    enemy_type,
                }),
                _ => ()
            }
        }
    }

    cmd
        .spawn(TransformBundle::from_transform(Transform::from_xyz(
            330., 20., 0.,
        )))
        .insert(LevelEntry)
        .insert(Name::new("Entry"));

    cmd
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-325., 260., 0.),
            texture: tex.portal.clone(),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(Vec2::new(95., 100.)),
                ..default()
            },
            ..default()
        })
        .insert(Collider::ball(40.))
        .insert(Sensor)
        .insert(LevelExit)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert(EcholocationHitColor(Color::LIME_GREEN))
        .insert(AgentRotation(-120.))
        .insert(Name::new("Exit"));

    // enemies
    // for (x, y, enemy_type) in [
    //     (-200., -100., EnemyType::FollowPing),
    //     (360., -250., EnemyType::Spiky),
    //     (200., -250., EnemyType::FollowPing),
    // ]
    // .iter()
    // {
    //     ev_w.send(SpawnEnemyEv {
    //         position: Vec2::new(*x, *y),
    //         enemy_type: *enemy_type,
    //     });
    // }
}

pub(super) fn update_score(mut ev_r: EventReader<PlayerEv>, mut reached: ResMut<ReachedLevel>) {
    for ev in ev_r.iter() {
        if let PlayerEv::ClearedLevel = ev {
            reached.0 += 1;
        }
    }
}
