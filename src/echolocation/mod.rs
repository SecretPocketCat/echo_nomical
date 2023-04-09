use bevy::prelude::*;

use crate::{assets::textures::TextureAssets, state::UnpausedGame};

use self::{
    echolocation::{echo_hit, echolocate, EcholocationHitEv},
    wave::spawn_wave,
};

pub mod echolocation;
pub mod wave;

pub fn echo_plugin(app: &mut App) {
    app.add_event::<EcholocationHitEv>()
        .add_systems((echolocate, echo_hit).in_set(UnpausedGame))
        .add_system(spawn_wave.run_if(resource_exists::<TextureAssets>()));
}
