use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{level::level::LevelSize, state::PersistReset, time::time::*};

#[derive(Component, Deref, DerefMut, Default)]
pub struct MovementDirection(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

/* How long does it take to damp the movement direction */
#[derive(Component, Deref, DerefMut, Default)]
pub struct Damping(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Rotation(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnParent(pub Entity);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Age(pub f32);

pub(super) fn move_agents(
    mut velocity_q: Query<(
        &MovementDirection,
        &Speed,
        &mut Transform,
        Option<&mut KinematicCharacterController>,
    )>,
    time: ScaledTime,
) {
    for (dir, speed, mut trans, char_cont) in velocity_q.iter_mut() {
        let vel = dir.0 * speed.0 * time.scaled_delta_seconds();

        if let Some(mut char_cont) = char_cont {
            char_cont.translation = Some(vel);
        } else {
            trans.translation += vel.extend(0.);
        }
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

// todo: look at target?
// pub(super) fn rotate_agent(
//     mut velocity_q: Query<(&LookAtDirection, &Speed, &mut Transform)>,
//     time: ScaledTime,
// ) {
//     for (dir, speed, mut trans) in velocity_q.iter_mut() {
//         trans.translation += dir.extend(0.) * speed.0 * time.scaled_delta_seconds();
//     }
// }

pub(super) fn rotate(mut dir_q: Query<(&mut Transform, &Rotation)>, time: ScaledTime) {
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
    level_size: Res<LevelSize>,
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
