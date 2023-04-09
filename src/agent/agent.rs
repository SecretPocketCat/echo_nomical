use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_rapier2d::prelude::*;

use crate::{
    physics::{check_collision_start, check_collision_start_pair},
    state::PersistReset,
    time::time::*,
    AppSize, EntityCommandsExt,
};

#[derive(Component, Deref, DerefMut, Default)]
pub struct MovementDirection(pub Vec2);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Speed(pub f32);

/* How long does it take to damp the movement direction */
#[derive(Component, Deref, DerefMut, Default)]
pub struct Damping(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Rotation(pub f32);

#[derive(Component)]
pub struct Bounce;

pub struct BounceEv {
    entity: Entity,
    position: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub struct ReenableCollider(Timer);

#[derive(Component)]
pub struct Bouncable;

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

pub(super) fn bounce(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bounce_q: Query<(&mut MovementDirection, &GlobalTransform), With<Bounce>>,
    bouncable_q: Query<(), With<Bouncable>>,
    phys_ctx: Res<RapierContext>,
) {
    let bounces: HashSet<_> = collision_events
        .iter()
        .filter_map(|ev| check_collision_start_pair(ev, &bounce_q, &bouncable_q))
        .map(|e| e.0)
        .collect();

    for e in bounces.iter() {
        if let Ok((mut dir, t)) = bounce_q.get_mut(*e) {
            if let Some(hit) = phys_ctx.cast_ray_and_get_normal(
                t.translation().truncate(),
                dir.0,
                1000.,
                true,
                QueryFilter::default().exclude_collider(*e),
            ) {
                // disable coll to prevent double collisions and re-enable it after a short delay
                cmd.entity(*e).try_insert((
                    ColliderDisabled,
                    ReenableCollider(Timer::from_seconds(0.05, TimerMode::Once)),
                ));
                // todo: this doesn's quite work
                let reflected_dir = dir.0 - 2. * dir.0.dot(hit.1.normal) * hit.1.normal;
                dir.0 = reflected_dir;
            }

            // todo: reflect dir
            // todo: reenable coll after a bit
            // todo: bounce ev
        }
    }
}

pub(super) fn reenable_collider(
    mut cmd: Commands,
    time: ScaledTime,
    mut enable_q: Query<(Entity, &mut ReenableCollider)>,
) {
    for (e, mut enable) in &mut enable_q {
        enable.tick(time.scaled_delta());

        if enable.just_finished() {
            cmd.entity(e)
                .remove::<ReenableCollider>()
                .remove::<ColliderDisabled>();
        }
    }
}
