use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn physics_plugin(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
}
