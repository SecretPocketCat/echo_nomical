use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::mapgen::{gen_map, TileType};
use crate::{
    agent::agent::AgentRotation, assets::textures::TextureAssets,
    echolocation::echolocation::EcholocationHitColor, enemy::SpawnEnemyEv, palette::COL_PORTAL,
    player::player::PlayerEv, render::camera::PrimaryCamera, AppSize,
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
    reached: Res<ReachedLevel>,
    bounds: Res<AppSize>,
    mut camera_transform: Query<&mut Transform, With<PrimaryCamera>>,
    tex: Res<TextureAssets>,
) {
    let tile_size = 40.0;
    let half_ts = tile_size / 2.;
    let map_scale = 16. + 6. * reached.0 as f32;
    let map_size = (Vec2::new(16.0f32, 12.0f32).normalize() * map_scale).as_ivec2();
    let physical_map_size = map_size.as_vec2() * tile_size;
    let map: super::mapgen::Map;
    loop {
        if let Some(good_map) = gen_map(map_size.x, map_size.y) {
            let wall_count = good_map
                .tiles
                .iter()
                .filter(|&x| x == &TileType::Wall)
                .count();
            if wall_count as f32 / (good_map.tiles.len() as f32) < 0.6 {
                map = good_map;
                break;
            }
        }
    }

    camera_transform.single_mut().translation =
        (Vec2::new(map.width as f32, map.height as f32) * half_ts).extend(999.9);
    let scale_factor = (physical_map_size / bounds.0).max_element();
    camera_transform.single_mut().scale = Vec2::splat(scale_factor).extend(1.0);

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
