use bevy::{prelude::*, transform::TransformSystem};

use self::agent::{age, despawn_out_of_bounds, r#move, rotate, wrap};

pub mod agent;

pub fn agent_plugin(app: &mut App) {
    app.add_systems(
        (rotate, r#move)
            .chain()
            .in_base_set(CoreSet::PostUpdate)
            .before(TransformSystem::TransformPropagate),
    )
    .add_system(rotate)
    .add_system(
        wrap.in_base_set(CoreSet::PostUpdate)
            .after(TransformSystem::TransformPropagate),
    )
    .add_system(age.in_base_set(CoreSet::PreUpdate))
    .add_system(despawn_out_of_bounds);
}
