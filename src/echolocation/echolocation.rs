use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::actions::PlayerAction;

pub(super) fn test_intersections(
    rapier_context: Res<RapierContext>,
    input_q: Query<(&ActionState<PlayerAction>, &GlobalTransform)>,
    coll_q: Query<(&Collider)>,
) {
    let shape = Collider::ball(300.);

    for (_, t) in input_q
        .iter()
        .filter(|(input, ..)| input.just_released(PlayerAction::Echo))
    {
        warn!("echo");
        rapier_context.intersections_with_shape(
            t.translation().truncate(),
            0.,
            &shape,
            QueryFilter::default(),
            |coll_e| {
                if let Ok(coll) = coll_q.get(coll_e) && let Some(cuboid_coll) = coll.as_cuboid()  {
                    // todo raycast at the coll
                    // cuboid_coll.half_extents().
                }
                // todo: raycast at each shape

                warn!("The entity {:?} intersects our shape.", coll_e);
                true // Return `false` instead if we want to stop searching for other colliders that contain this point.
            },
        );
    }
}
