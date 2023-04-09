use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{Animator, EaseFunction};
use rand::{thread_rng, Rng};

use crate::{
    agent::agent::{MovementDirection, MovementDirectionEasing, Speed, StopOnCollision},
    animation::{delay_tween, get_relative_sprite_color_tween, TweenDoneAction},
    assets::textures::TextureAssets,
    echolocation::echolocation::{EcholocationHitColor, EcholocationHitEv, FollowEchoOnHit},
    input::cooldown::{process_cooldown, Cooldown},
    level::level::Wall,
    state::UnpausedGame,
    EntityCommandsExt,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_system(spawn_enemy.in_set(UnpausedGame))
        .add_systems((follow_echolocation, flash_on_echolocation))
        .add_system(process_cooldown::<FollowEchoOnHit>);
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

fn spawn_enemy(mut ev_r: EventReader<SpawnEnemyEv>, mut cmd: Commands, tex: Res<TextureAssets>) {
    if !ev_r.is_empty() {
        let mut rng = thread_rng();

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
                        .insert(sprite_bundle)
                        .insert(Collider::polyline(vert, None))
                        .insert(Name::new("Spiky"))
                        .insert(EcholocationHitColor(Color::CRIMSON));
                }
                EnemyType::FollowPing => {
                    let radius = 25.;
                    sprite_bundle.texture = tex.player.clone();
                    sprite_bundle.sprite.custom_size = Some(Vec2::splat(radius * 2.));

                    cmd.entity(e)
                        .insert(sprite_bundle)
                        .insert(Collider::ball(radius * 0.85))
                        .insert(Name::new("FolowPing"))
                        .insert(EcholocationHitColor(Color::CRIMSON))
                        .insert(FollowEchoOnHit)
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

pub(super) fn flash_on_echolocation(
    mut cmd: Commands,
    mut echo_hit_r: EventReader<EcholocationHitEv>,
    color_q: Query<&EcholocationHitColor, With<Sprite>>,
) {
    for ev in echo_hit_r.iter() {
        if let Ok(col) = color_q.get(ev.hit_e) {
            cmd.entity(ev.hit_e).try_insert(Animator::new(
                get_relative_sprite_color_tween(col.0, 250, TweenDoneAction::None).then(
                    delay_tween(
                        get_relative_sprite_color_tween(Color::NONE, 800, TweenDoneAction::None),
                        600,
                    ),
                ),
            ));
        }
    }
}

pub(super) fn follow_echolocation(
    mut cmd: Commands,
    mut echo_hit_r: EventReader<EcholocationHitEv>,
    mut follow_q: Query<
        &mut MovementDirection,
        (With<FollowEchoOnHit>, Without<Cooldown<FollowEchoOnHit>>),
    >,
) {
    for ev in echo_hit_r.iter() {
        if let Ok(mut dir) = follow_q.get_mut(ev.hit_e) {
            dir.0 = ev.direction.normalize_or_zero();
            cmd.entity(ev.hit_e)
                .try_insert(Cooldown::<FollowEchoOnHit>::new(0.5));
        }
    }
}
