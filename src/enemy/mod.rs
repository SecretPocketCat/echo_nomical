use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{echolocation::echolocation::EcholocationHitColor, state::UnpausedGame};

pub fn enemy_plugin(app: &mut App) {
    app.add_event::<SpawnEnemyEv>()
        .add_system(spawn_enemy.in_set(UnpausedGame));
}

#[derive(Component)]
pub struct Enemy;

#[derive(Deref, DerefMut)]
pub struct SpawnEnemyEv(pub Vec2);

fn spawn_enemy(mut ev_r: EventReader<SpawnEnemyEv>, mut cmd: Commands) {
    for ev in ev_r.iter() {
        let radius = 30.;
        cmd.spawn(SpatialBundle::from_transform(Transform::from_translation(
            ev.extend(0.),
        )))
        .insert(Collider::ball(radius * 0.7))
        .insert(Sensor)
        .insert(Enemy)
        .insert(Name::new("Enemy"))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert(EcholocationHitColor(Color::CRIMSON));
    }
}
