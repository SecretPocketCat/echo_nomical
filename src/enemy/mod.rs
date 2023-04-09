use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{echolocation::echolocation::EcholocationHitColor, state::UnpausedGame};

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_system(spawn_enemy.in_set(UnpausedGame));
}

#[derive(Debug, Component, Clone, Copy)]
pub enum EnemyType {
    Spiky,
    Bouncy,
    Dasher,
    // Walker,
    // Elite,
}

#[derive(Debug)]
pub struct SpawnEnemyEv {
    pub position: Vec2,
    pub enemy_type: EnemyType,
}

fn spawn_enemy(mut ev_r: EventReader<SpawnEnemyEv>, mut cmd: Commands) {
    if !ev_r.is_empty() {
        let mut rng = thread_rng();

        for ev in ev_r.iter() {
            match ev.enemy_type {
                EnemyType::Spiky => {
                    let radius = rng.gen_range(40.0..70.);
                    let spike_count = 12;
                    let ray_step = 360. / (spike_count * 2) as f32;

                    let vert = (0..=(spike_count * 2))
                        .map(|i| {
                            let dir = Vec2::from_angle((i as f32 * ray_step).to_radians());
                            if i % 2 == 0 {
                                dir * radius
                            } else {
                                dir * radius * 0.65
                            }
                        })
                        .collect();
                    cmd.spawn(SpatialBundle::from_transform(Transform::from_translation(
                        ev.position.extend(0.),
                    )))
                    .insert(Collider::polyline(vert, None))
                    .insert(Sensor)
                    .insert(ev.enemy_type)
                    .insert(Name::new("Enemy"))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ActiveCollisionTypes::all())
                    .insert(EcholocationHitColor(Color::CRIMSON));
                }
                EnemyType::Bouncy => {}
                EnemyType::Dasher => {}
            }
        }
    }
}
