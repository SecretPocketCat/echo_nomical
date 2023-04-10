use bevy::{prelude::*, transform::TransformSystem};

use crate::state::UnpausedGame;

use self::agent::{
    age, apply_damping, despawn_out_of_bounds, ease_direction, move_agents, rotate,
    stop_on_wall_collision, MovementDirection, MovementDirectionEasing,
};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    app.add_systems(
        (rotate, apply_damping, ease_direction, move_agents)
            .chain()
            .in_set(UnpausedGame)
            .in_base_set(CoreSet::PostUpdate)
            .before(TransformSystem::TransformPropagate),
    )
    .add_systems((stop_on_wall_collision, despawn_out_of_bounds).in_set(UnpausedGame))
    .add_system(age.in_base_set(CoreSet::PreUpdate));

    app.register_type::<MovementDirection>()
        .register_type::<MovementDirectionEasing>();
}
