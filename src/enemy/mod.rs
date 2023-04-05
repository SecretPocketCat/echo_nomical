use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::common_conditions::on_timer};

use crate::{
    agent::agent::{Age, MovementDirection, Speed},
    projectile::{
        spawner::{ProjectileSpawner, SpawnInterval, SpawnerType},
        ProjectilePath,
    },
    state::AppState,
};

pub fn enemy_plugin(app: &mut App) {
    app.add_system(setup_enemies.in_schedule(OnEnter(AppState::Game)));
}

// no procgen for now : just hardcoded enemies to try it out
fn setup_enemies(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // todo: just moving boxes with colliders for now
}
