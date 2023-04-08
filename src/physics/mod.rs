use bevy::{
    ecs::query::{ReadOnlyWorldQuery, WorldQuery},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

pub fn physics_plugin(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
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
) -> bool {
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
) -> bool {
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
) -> bool {
    check_collision_pair_with_type(CollisionEventType::Start, collision, q_1, q_2)
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
) -> bool {
    let any = event_type == CollisionEventType::Any;
    if let Some((e1, e2)) = match collision {
        CollisionEvent::Started(e1, e2, _) if (any || event_type == CollisionEventType::Start) => {
            Some((e1.clone(), e2.clone()))
        }
        CollisionEvent::Stopped(e1, e2, _) if (any || event_type == CollisionEventType::End) => {
            Some((e1.clone(), e2.clone()))
        }
        _ => None,
    } {
        (q_1.contains(e1) && q_2.contains(e2)) || (q_1.contains(e2) && q_2.contains(e1))
    } else {
        false
    }
}
