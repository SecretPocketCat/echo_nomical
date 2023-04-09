use bevy::{prelude::*, transform::TransformSystem};

use crate::state::UnpausedGame;

use self::agent::{age, apply_damping, bounce, despawn_out_of_bounds, move_agents, rotate};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    app.add_systems(
        (rotate, apply_damping, move_agents, bounce)
            .chain()
            .in_set(UnpausedGame)
            .in_base_set(CoreSet::PostUpdate)
            .before(TransformSystem::TransformPropagate),
    )
    .add_system(age.in_base_set(CoreSet::PreUpdate))
    .add_system(despawn_out_of_bounds);
}
