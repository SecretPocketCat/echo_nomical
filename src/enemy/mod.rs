use bevy::{prelude::*};

use crate::{
    state::AppState,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_system(setup_enemies.in_schedule(OnEnter(AppState::Game)));
}

// no procgen for now : just hardcoded enemies to try it out
fn setup_enemies(
    _cmd: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    // todo: just moving boxes with colliders for now
}
