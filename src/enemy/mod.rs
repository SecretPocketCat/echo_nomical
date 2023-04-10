use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::TransformRotateZLens, Animator, EaseFunction, Tween};
use rand::thread_rng;

use crate::{
    agent::agent::{MovementDirection, MovementDirectionEasing, Speed, StopOnCollision},
    animation::{get_relative_scale_anim, TweenDoneAction},
    assets::textures::TextureAssets,
    echolocation::{
        echolocation::{EcholocationHitColor, EcholocationHitEv, FollowEchoOnHit},
        wave::Wave,
    },
    input::cooldown::{process_cooldown, Cooldown},
    level::level::Wall,
    palette::COL_ENEMY,
    physics::check_collision_start_pair,
    state::UnpausedGame,
    EntityCommandsExt,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_event::<EnemyEv>()
        .add_systems((spawn_enemy, enemy_hit).in_set(UnpausedGame))
        .add_system(follow_echolocation)
        .add_system(process_cooldown::<FollowEchoOnHit>);
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Spiky,
    FollowPing,
}

#[derive(Component)]
pub struct Killable;

#[derive(Component)]
pub struct Killer;

#[derive(Debug)]
pub struct SpawnEnemyEv {
    pub position: Vec2,
    pub enemy_type: EnemyType,
}

pub enum EnemyEv {
    Killed,
    Alarmed,
}

fn spawn_enemy(mut ev_r: EventReader<SpawnEnemyEv>, mut cmd: Commands, tex: Res<TextureAssets>) {
    if !ev_r.is_empty() {
        let _rng = thread_rng();

        for ev in ev_r.iter() {
            let mut sprite_bundle = SpriteBundle {
                transform: Transform::from_translation(ev.position.extend(0.)),
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                ..default()
            };

            let e = cmd
                .spawn(Sensor)
                .insert(ev.enemy_type)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ActiveCollisionTypes::all())
                .id();

            match ev.enemy_type {
                EnemyType::Spiky => {
                    let radius = 20.;
                    let spike_count = 12;
                    let ray_step = 360. / (spike_count * 2) as f32;

                    sprite_bundle.texture = tex.spiky.clone();
                    sprite_bundle.sprite.custom_size = Some(Vec2::splat(radius * 2.));

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
                        .insert(sprite_bundle)
                        .insert(Collider::polyline(vert, None))
                        .insert(Name::new("Spiky"))
                        .insert(Killer)
                        .insert(EcholocationHitColor(COL_ENEMY));
                }
                EnemyType::FollowPing => {
                    let half_width = 15.;
                    let height = 30.;
                    let btm = 5.;
                    sprite_bundle.texture = tex.charge.clone();
                    sprite_bundle.sprite.custom_size =
                        Some(Vec2::new(half_width * 2.25, height * 1.125));
                    sprite_bundle.sprite.anchor = Anchor::BottomCenter;
                    sprite_bundle.transform.translation.y -= height / 2.;

                    let verts = vec![
                        (-half_width, btm),
                        (half_width, btm),
                        (0., height),
                        (-half_width, btm),
                    ]
                    .iter()
                    .map(|(x, y)| Vec2::new(*x, *y))
                    .collect();

                    cmd.entity(e)
                        .insert(sprite_bundle)
                        .insert(Collider::polyline(verts, None))
                        .insert(Name::new("FolowPing"))
                        .insert(EcholocationHitColor(Color::CRIMSON))
                        .insert(FollowEchoOnHit)
                        .insert(Killable)
                        .insert(Killer)
                        .insert(StopOnCollision::<Wall>::new())
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

pub(super) fn follow_echolocation(
    mut cmd: Commands,
    mut echo_hit_r: EventReader<EcholocationHitEv>,
    mut follow_q: Query<
        (
            &mut MovementDirection,
            Option<&mut MovementDirectionEasing>,
            &Transform,
        ),
        (With<FollowEchoOnHit>, Without<Cooldown<FollowEchoOnHit>>),
    >,
    mut enemy_ev_w: EventWriter<EnemyEv>,
) {
    for ev in echo_hit_r.iter() {
        if let Ok((mut dir, dir_easing, t)) = follow_q.get_mut(ev.hit_e) {
            enemy_ev_w.send(EnemyEv::Alarmed);
            cmd.entity(ev.hit_e)
                .try_insert(Cooldown::<FollowEchoOnHit>::new(1.25));

            let dir_norm = ev.direction.normalize_or_zero();

            // look at dir
            cmd.entity(ev.hit_e).try_insert(Animator::new(
                Tween::new(
                    EaseFunction::QuadraticInOut,
                    Duration::from_millis(400),
                    TransformRotateZLens {
                        start: t.rotation.to_euler(EulerRot::XYZ).2,
                        end: Vec2::Y.angle_between(dir_norm),
                    },
                )
                .with_completed_event(TweenDoneAction::None),
            ));
            dir.0 = dir_norm;

            if let Some(mut dir_ease) = dir_easing {
                dir_ease.reset();
            }
        }
    }
}

pub(super) fn enemy_hit(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    killable_q: Query<(), With<Killable>>,
    killer_q: Query<(), With<Killer>>,
    wave_data_q: Query<(
        &GlobalTransform,
        Option<&EcholocationHitColor>,
        Option<&MovementDirection>,
    )>,
    mut enemy_ev_w: EventWriter<EnemyEv>,
) {
    for coll in collision_events
        .iter()
        .filter_map(|ev| check_collision_start_pair(ev, &killable_q, &killer_q))
    {
        let mut killable = vec![coll.0];

        if killer_q.contains(coll.0) && killer_q.contains(coll.1) {
            // both are killable
            killable.push(coll.1);
        }

        enemy_ev_w.send(EnemyEv::Killed);

        for e in killable.iter() {
            cmd.entity(*e)
                .try_insert(ColliderDisabled)
                .try_insert(get_relative_scale_anim(
                    Vec2::ZERO.extend(1.),
                    300,
                    TweenDoneAction::DespawnSelfRecursive,
                ));

            if let Ok((t, color, dir)) = wave_data_q.get(*e) {
                cmd.spawn(Wave {
                    position: t.translation() + dir.map_or(Vec2::ZERO, |d| d.0 * 50.).extend(0.),
                    radius: 80.,
                    color: color.map_or(COL_ENEMY, |c| c.0),
                });
            }
        }
    }
}
