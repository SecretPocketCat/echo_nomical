use bevy::prelude::*;

use crate::{level::level::LevelSize, time::time::*};

#[derive(Component, Deref, DerefMut, Default)]
pub struct MovementDirection(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Rotation(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnParent(pub Entity);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Age(pub f32);

pub(super) fn r#move(
    mut velocity_q: Query<(&MovementDirection, &Speed, &mut Transform)>,
    time: ScaledTime,
) {
    for (dir, speed, mut trans) in velocity_q.iter_mut() {
        trans.translation += dir.extend(0.) * speed.0 * time.scaled_delta_seconds();
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
    despawn_q: Query<(Entity, &GlobalTransform, Option<&DespawnParent>)>,
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
