use bevy::{
    ecs::query::{ReadOnlyWorldQuery, WorldQuery},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

pub fn physics_plugin(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
}

pub const ECHO_COLL_GROUP: Group = Group::GROUP_1;
pub const PLAYER_COLL_GROUP: Group = Group::GROUP_2;

pub struct CollisionSuccess {
    pub hit: Entity,
    pub other: Entity,
}

#[allow(dead_code)]
pub fn check_collision_pair<
    TW1: WorldQuery,
    TRW1: ReadOnlyWorldQuery,
    TW2: WorldQuery,
    TRW2: ReadOnlyWorldQuery,
>(
    collision: &CollisionEvent,
    q_1: &Query<TW1, TRW1>,
    q_2: &Query<TW2, TRW2>,
) -> Option<(Entity, Entity)> {
    check_collision_pair_with_type(CollisionEventType::Any, collision, q_1, q_2)
}

#[allow(dead_code)]
pub fn check_collision_end_pair<
    TW1: WorldQuery,
    TRW1: ReadOnlyWorldQuery,
    TW2: WorldQuery,
    TRW2: ReadOnlyWorldQuery,
>(
    collision: &CollisionEvent,
    q_1: &Query<TW1, TRW1>,
    q_2: &Query<TW2, TRW2>,
) -> Option<(Entity, Entity)> {
    check_collision_pair_with_type(CollisionEventType::End, collision, q_1, q_2)
}

#[allow(dead_code)]
pub fn check_collision_start_pair<
    TW1: WorldQuery,
    TRW1: ReadOnlyWorldQuery,
    TW2: WorldQuery,
    TRW2: ReadOnlyWorldQuery,
>(
    collision: &CollisionEvent,
    q_1: &Query<TW1, TRW1>,
    q_2: &Query<TW2, TRW2>,
) -> Option<(Entity, Entity)> {
    check_collision_pair_with_type(CollisionEventType::Start, collision, q_1, q_2)
}

#[allow(dead_code)]
pub fn check_collision<TW: WorldQuery, TRW: ReadOnlyWorldQuery>(
    collision: &CollisionEvent,
    q: &Query<TW, TRW>,
) -> Option<CollisionSuccess> {
    check_collision_with_type(CollisionEventType::Any, collision, q)
}

#[allow(dead_code)]
pub fn check_collision_start<TW: WorldQuery, TRW: ReadOnlyWorldQuery>(
    collision: &CollisionEvent,
    q: &Query<TW, TRW>,
) -> Option<CollisionSuccess> {
    check_collision_with_type(CollisionEventType::Start, collision, q)
}

#[allow(dead_code)]
pub fn check_collision_end<TW: WorldQuery, TRW: ReadOnlyWorldQuery>(
    collision: &CollisionEvent,
    q: &Query<TW, TRW>,
) -> Option<CollisionSuccess> {
    check_collision_with_type(CollisionEventType::End, collision, q)
}

#[derive(PartialEq, Eq)]
enum CollisionEventType {
    Any,
    Start,
    End,
}

fn check_collision_pair_with_type<
    TW1: WorldQuery,
    TRW1: ReadOnlyWorldQuery,
    TW2: WorldQuery,
    TRW2: ReadOnlyWorldQuery,
>(
    event_type: CollisionEventType,
    collision: &CollisionEvent,
    q_1: &Query<TW1, TRW1>,
    q_2: &Query<TW2, TRW2>,
) -> Option<(Entity, Entity)> {
    if let Some((e1, e2)) = get_collision_pair(event_type, collision) {
        if q_1.contains(e1) && q_2.contains(e2) {
            Some((e1, e2))
        } else if q_1.contains(e2) && q_2.contains(e1) {
            Some((e2, e1))
        } else {
            None
        }
    } else {
        None
    }
}

fn check_collision_with_type<TW: WorldQuery, TRW: ReadOnlyWorldQuery>(
    event_type: CollisionEventType,
    collision: &CollisionEvent,
    q: &Query<TW, TRW>,
) -> Option<CollisionSuccess> {
    if let Some((e1, e2)) = get_collision_pair(event_type, collision) {
        if q.contains(e1) {
            Some(CollisionSuccess { hit: e1, other: e2 })
        } else if q.contains(e2) {
            Some(CollisionSuccess { hit: e2, other: e1 })
        } else {
            None
        }
    } else {
        None
    }
}

fn get_collision_pair(
    event_type: CollisionEventType,
    collision: &CollisionEvent,
) -> Option<(Entity, Entity)> {
    let any = event_type == CollisionEventType::Any;
    match collision {
        CollisionEvent::Started(e1, e2, _) if (any || event_type == CollisionEventType::Start) => {
            Some((e1.clone(), e2.clone()))
        }
        CollisionEvent::Stopped(e1, e2, _) if (any || event_type == CollisionEventType::End) => {
            Some((e1.clone(), e2.clone()))
        }
        _ => None,
    }
}
