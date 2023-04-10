use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use interpolation::*;

use crate::{
    level::level::Wall,
    physics::{check_collision_start, check_collision_start_pair},
    state::PersistReset,
    time::time::*,
    AppSize,
};

#[derive(Component, Deref, DerefMut, Default, Reflect)]
pub struct MovementDirection(pub Vec2);

#[derive(Component, Reflect)]
pub struct MovementDirectionEasing {
    time_to_ease: f32,
    time: f32,
    #[reflect(ignore)]
    ease: EaseFunction,
    eased_dir: Vec2,
}

impl MovementDirectionEasing {
    pub fn new(time_to_ease: f32) -> Self {
        Self::with_ease_fn(time_to_ease, EaseFunction::SineInOut)
    }

    pub fn with_ease_fn(time_to_ease: f32, ease: EaseFunction) -> Self {
        Self {
            time_to_ease,
            time: 0.,
            ease,
            eased_dir: Vec2::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.time = 0.
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

/* How long does it take to damp the movement direction */
#[derive(Component, Deref, DerefMut, Default)]
pub struct Damping(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct AgentRotation(pub f32);

#[derive(Component)]
pub struct StopOnCollision<T>(PhantomData<T>);

impl<T> StopOnCollision<T> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DespawnParent(pub Entity);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Age(pub f32);

pub(super) fn move_agents(
    mut velocity_q: Query<(
        &MovementDirection,
        Option<&MovementDirectionEasing>,
        &Speed,
        &mut Transform,
        Option<&mut KinematicCharacterController>,
    )>,
    time: ScaledTime,
) {
    for (dir, eased, speed, mut trans, char_cont) in velocity_q.iter_mut() {
        let vel = eased.map_or(dir.0, |e| e.eased_dir) * speed.0 * time.scaled_delta_seconds();

        if let Some(mut char_cont) = char_cont {
            char_cont.translation = Some(vel);
        } else {
            trans.translation += vel.extend(0.);
        }
    }
}

pub(super) fn ease_direction(
    mut dir_easing_q: Query<(&MovementDirection, &mut MovementDirectionEasing)>,
    time: ScaledTime,
) {
    for (dir, mut ease_dir) in dir_easing_q.iter_mut() {
        let time_step = time.scaled_delta_seconds()
            * if ease_dir.eased_dir.length() < dir.0.length() {
                1.
            } else {
                -1.
            };

        ease_dir.time = (ease_dir.time + time_step).clamp(0., ease_dir.time_to_ease);
        ease_dir.eased_dir = dir.0 * ((ease_dir.time / ease_dir.time_to_ease).calc(ease_dir.ease));
    }
}

pub(super) fn apply_damping(
    mut damping_q: Query<(&mut MovementDirection, &Damping)>,
    time: ScaledTime,
) {
    for (mut dir, damping) in damping_q.iter_mut() {
        let damping = dir.0 * (1. / damping.0) * time.scaled_delta_seconds();
        dir.0 -= damping;
    }
}

pub(super) fn rotate(mut dir_q: Query<(&mut Transform, &AgentRotation)>, time: ScaledTime) {
    for (mut t, rotation) in &mut dir_q {
        t.rotate_local_z((rotation.0 * time.scaled_delta_seconds()).to_radians());
    }
}

pub(super) fn age(mut age_q: Query<&mut Age>, time: ScaledTime) {
    for mut age in &mut age_q.iter_mut() {
        age.0 += time.scaled_delta_seconds();
    }
}

pub(super) fn despawn_out_of_bounds(
    despawn_q: Query<(Entity, &GlobalTransform, Option<&DespawnParent>), Without<PersistReset>>,
    level_size: Res<AppSize>,
    mut cmd: Commands,
) {
    for (e, t, despawn_parent) in despawn_q.iter() {
        let pos = t.translation().abs();
        if pos.x > level_size.x || pos.y > level_size.y {
            if let Some(e_cmd) = cmd.get_entity(despawn_parent.map_or(e, |e| e.0)) {
                e_cmd.despawn_recursive();
            }
        }
    }
}

pub(super) fn stop_on_wall_collision(
    mut collision_events: EventReader<CollisionEvent>,
    stoppable_q: Query<(), With<StopOnCollision<Wall>>>,
    stop_q: Query<(), With<Wall>>,
    mut dir_q: Query<&mut MovementDirection>,
) {
    for coll in collision_events
        .iter()
        .filter_map(|ev| check_collision_start_pair(ev, &stoppable_q, &stop_q))
    {
        if let Ok(mut dir) = dir_q.get_mut(coll.0) && dir.0.length() > 0. {
            dir.0 = Vec2::ZERO;
        }
    }
}
