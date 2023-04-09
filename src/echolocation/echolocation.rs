use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{Animator, Sequence};
use leafwing_input_manager::prelude::*;

use crate::{
    agent::agent::{Age, Damping, MovementDirection, Speed},
    animation::{
        get_relative_scale_anim, get_relative_scale_tween, get_relative_sprite_color_anim,
        TweenDoneAction,
    },
    assets::textures::TextureAssets,
    input::actions::PlayerAction,
    physics::{check_collision_start, ECHO_COLL_GROUP, PLAYER_COLL_GROUP},
    EntityCommandsExt,
};

const ECHO_RAY_MAX_AGE_MS: u64 = 2500;
const ECHO_RAY_MAX_AGE_S: f32 = (ECHO_RAY_MAX_AGE_MS / 1000) as f32;

#[derive(Component)]
pub(super) struct EcholocationRay;

#[derive(Component)]
pub struct EcholocationHitColor(pub Color);

pub(super) fn echolocate(
    mut cmd: Commands,
    input_q: Query<(&ActionState<PlayerAction>, &GlobalTransform)>,
    textures: Res<TextureAssets>,
) {
    for (_, t) in input_q
        .iter()
        .filter(|(input, ..)| input.just_pressed(PlayerAction::Echo))
    {
        let ray_count = 100;
        let ray_step = 360. / ray_count as f32;
        let pos = t.translation();
        let radius = 3.;

        for i in 0..ray_count {
            let dir = Vec2::from_angle((i as f32 * ray_step).to_radians());

            cmd.spawn(SpatialBundle::from_transform(Transform::from_translation(
                pos + dir.extend(0.) * 10.,
            )))
            .insert(EcholocationRay)
            .insert(MovementDirection(dir))
            .insert(Speed(180.))
            .insert(Damping(ECHO_RAY_MAX_AGE_S - 0.1))
            .insert(Age::default())
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::ball(radius))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ActiveCollisionTypes::all())
            .insert(CollisionGroups::new(
                ECHO_COLL_GROUP.into(),
                Group::all()
                    .difference(ECHO_COLL_GROUP)
                    .difference(PLAYER_COLL_GROUP),
            ))
            .with_children(|b| {
                let parent_e = b.parent_entity();

                b.spawn(SpriteBundle {
                    transform: Transform::from_scale(Vec2::ZERO.extend(1.)),
                    texture: textures.echo_ping.clone(),
                    sprite: Sprite {
                        color: Color::SEA_GREEN,
                        custom_size: Some(Vec2::splat(8.)),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(get_relative_scale_anim(
                    Vec3::ONE,
                    180,
                    TweenDoneAction::None,
                ))
                .insert(get_relative_sprite_color_anim(
                    Color::NONE,
                    ECHO_RAY_MAX_AGE_MS,
                    TweenDoneAction::DespawnRecursive(parent_e),
                ));
            });
        }
    }
}

pub(super) fn echo_hit(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    ray_q: Query<(), With<EcholocationRay>>,
    trans_q: Query<(&GlobalTransform, &Age)>,
    color_q: Query<&EcholocationHitColor>,
    mut move_q: Query<&mut MovementDirection>,
    textures: Res<TextureAssets>,
    time: Res<Time>,
) {
    for coll_success in collision_events
        .iter()
        .filter_map(|ev| check_collision_start(ev, &ray_q))
    {
        cmd.entity(coll_success.hit)
            .try_insert(ColliderDisabled)
            .try_insert(get_relative_scale_anim(
                Vec2::ZERO.extend(1.),
                150,
                TweenDoneAction::DespawnSelfRecursive,
            ));

        if let Ok(mut dir) = move_q.get_mut(coll_success.hit) {
            dir.0 = Vec2::ZERO;
        }

        if let Ok((t, age)) = trans_q.get(coll_success.hit) {
            let age_mult = ((ECHO_RAY_MAX_AGE_S - age.0 - 0.1) / ECHO_RAY_MAX_AGE_S).min(1.);

            if age_mult > 0. {
                let col = color_q
                    .get(coll_success.other)
                    .map_or(Color::SEA_GREEN, |c| c.0);

                cmd.spawn(SpriteBundle {
                    transform: Transform::from_translation(
                        t.translation()
                            .truncate()
                            .extend(time.elapsed_seconds_wrapped() / 10000.),
                    )
                    .with_scale(Vec2::ZERO.extend(1.)),
                    texture: textures.echo_ping.clone(),
                    sprite: Sprite {
                        color: col,
                        custom_size: Some(Vec2::splat(20.) * age_mult),
                        ..default()
                    },
                    ..Default::default()
                })
                .try_insert(Animator::new(Sequence::new(vec![
                    get_relative_scale_tween(Vec3::ONE, 150, TweenDoneAction::None),
                    get_relative_scale_tween(
                        Vec3::ZERO,
                        (4000. * age_mult) as u64,
                        TweenDoneAction::None,
                    ),
                ])))
                .try_insert(get_relative_sprite_color_anim(
                    Color::NONE,
                    (4200. * age_mult) as u64,
                    TweenDoneAction::DespawnSelfRecursive,
                ));
            }
        }
    }
}
