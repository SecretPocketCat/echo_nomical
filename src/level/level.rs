use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::map::TileType;
use crate::{
    agent::agent::AgentRotation, assets::textures::TextureAssets,
    echolocation::echolocation::EcholocationHitColor, enemy::SpawnEnemyEv, palette::COL_PORTAL,
    player::player::PlayerEv,
};

#[derive(Component)]
pub struct LevelEntry;

#[derive(Component)]
pub struct LevelExit;

#[derive(Resource, Default)]
pub struct ReachedLevel(pub usize);

#[derive(Component)]
pub struct Wall;

pub(super) fn setup_level(
    mut cmd: Commands,
    mut ev_w: EventWriter<SpawnEnemyEv>,
    tex: Res<TextureAssets>,
    map: Res<super::Map>,
) {
    let tile_size = super::TILE_SIZE;
    let half_ts = tile_size / 2.;

    let mut spawned_player = false;

    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.xy(x, y);
            let (x, y) = (x as f32, y as f32);
            let x = tile_size * x + half_ts;
            let y = tile_size * y + half_ts;
            match tile {
                &TileType::Wall => {
                    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
                        x, y, 0.,
                    )))
                    .insert(Collider::cuboid(half_ts, half_ts))
                    .insert(Wall)
                    .insert(Name::new("Wall"));
                }
                &TileType::Enemy(enemy_type) => ev_w.send(SpawnEnemyEv {
                    position: Vec2::new(x, y),
                    enemy_type,
                }),
                &TileType::Goal => {
                    cmd.spawn(SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.),
                        texture: tex.portal.clone(),
                        sprite: Sprite {
                            color: Color::NONE,
                            custom_size: Some(Vec2::new(95., 100.)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Collider::ball(20.))
                    .insert(Sensor)
                    .insert(LevelExit)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ActiveCollisionTypes::all())
                    .insert(EcholocationHitColor(COL_PORTAL))
                    .insert(AgentRotation(-120.))
                    .insert(Name::new("Exit"));
                }
                &TileType::PlayerSpawn => {
                    if spawned_player {
                        continue;
                    }
                    spawned_player = true;
                    cmd.spawn(TransformBundle::from_transform(Transform::from_xyz(
                        x, y, 0.,
                    )))
                    .insert(LevelEntry)
                    .insert(Name::new("Entry"));
                }
                _ => {}
            }
        }
    }
}

pub(super) fn update_score(mut ev_r: EventReader<PlayerEv>, mut reached: ResMut<ReachedLevel>) {
    for ev in ev_r.iter() {
        if let PlayerEv::ClearedLevel = ev {
            reached.0 += 1;
        }
    }
}
