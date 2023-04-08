use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::state::UnpausedGame;

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_system(spawn_enemy.in_set(UnpausedGame));
}

#[derive(Component)]
pub struct Enemy;

#[derive(Deref, DerefMut)]
pub struct SpawnEnemyEv(pub Vec2);

fn spawn_enemy(
    mut ev_r: EventReader<SpawnEnemyEv>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in ev_r.iter() {
        let radius = 30.;
        cmd.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(ev.extend(0.)),
            ..default()
        })
        .insert(Collider::ball(radius * 0.7))
        .insert(Sensor)
        .insert(Enemy)
        .insert(Name::new("Enemy"))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all());
    }
}
