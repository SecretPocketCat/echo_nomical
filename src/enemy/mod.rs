use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    agent::agent::{Bounce, MovementDirection, Speed},
    echolocation::echolocation::EcholocationHitColor,
    state::UnpausedGame,
};

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

                    cmd.entity(e)
                        .insert(Collider::polyline(vert, None))
                        .insert(Name::new("Spiky"))
                        .insert(EcholocationHitColor(Color::CRIMSON));
                }
                EnemyType::Bouncy => {
                    cmd.entity(e)
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(Collider::ball(rng.gen_range(25.0..40.)))
                        .insert(Name::new("Bouncy"))
                        .insert(EcholocationHitColor(Color::ORANGE_RED))
                        .insert(Speed(50.))
                        .insert(MovementDirection(
                            Vec2::new(rng.gen_range(-1.1..=1.), rng.gen_range(-1.0..=1.))
                                .normalize_or_zero(),
                        ))
                        .insert(Bounce);
                }
                EnemyType::Dasher => {}
            }
        }
    }
}
