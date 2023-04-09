use bevy::{prelude::*, transform::TransformSystem};

use crate::state::UnpausedGame;

use self::agent::{age, apply_damping, despawn_out_of_bounds, ease_direction, move_agents, rotate};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    app.add_systems(
        (rotate, apply_damping, ease_direction, move_agents)
            .chain()
            .in_set(UnpausedGame)
            .in_base_set(CoreSet::PostUpdate)
            .before(TransformSystem::TransformPropagate),
    )
    .add_system(age.in_base_set(CoreSet::PreUpdate))
    .add_system(despawn_out_of_bounds);
}
