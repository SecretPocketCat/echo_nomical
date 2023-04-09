use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::EaseFunction;
use rand::{thread_rng, Rng};

use crate::{
    agent::agent::{MovementDirection, MovementDirectionEasing, Speed},
    echolocation::echolocation::{EcholocationHitColor, EcholocationHitEv, FollowEchoOnHit},
    state::UnpausedGame,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_system(spawn_enemy.in_set(UnpausedGame))
        .add_systems((follow_echolocation, flash_on_echolocation));
}

#[derive(Debug, Component, Clone, Copy)]
pub enum EnemyType {
    Static,
    FollowPing,
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
            let e = cmd
                .spawn(SpatialBundle::from_transform(Transform::from_translation(
                    ev.position.extend(0.),
                )))
                .insert(Sensor)
                .insert(ev.enemy_type)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ActiveCollisionTypes::all())
                .id();

            match ev.enemy_type {
                EnemyType::Static => {
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

                    cmd.entity(e)
                        .insert(Collider::polyline(vert, None))
                        .insert(Name::new("Spiky"))
                        .insert(EcholocationHitColor(Color::CRIMSON));
                }
                EnemyType::FollowPing => {
                    cmd.entity(e)
                        .insert(Collider::ball(30.))
                        .insert(Name::new("FolowPing"))
                        .insert(EcholocationHitColor(Color::CRIMSON))
                        .insert(FollowEchoOnHit)
                        .insert(Speed(220.))
                        .insert(MovementDirection::default())
                        .insert(MovementDirectionEasing::with_ease_fn(
                            1.5,
                            EaseFunction::QuadraticIn,
                        ));
                }
            }
        }
    }
}

pub(super) fn flash_on_echolocation(mut echo_hit_r: EventReader<EcholocationHitEv>) {}

pub(super) fn follow_echolocation(
    mut echo_hit_r: EventReader<EcholocationHitEv>,
    mut follow_q: Query<&mut MovementDirection, With<FollowEchoOnHit>>,
) {
    for ev in echo_hit_r.iter() {
        if let Ok(mut dir) = follow_q.get_mut(ev.hit_e) {
            dir.0 = ev.direction;
        }
    }
}
