use bevy::prelude::*;

use crate::{assets::textures::TextureAssets, state::UnpausedGame};

use self::{
    echolocation::{
        echo_hit, echolocate, flash_on_echolocation, EcholocationEv, EcholocationHitEv,
    },
    wave::spawn_wave,
};

pub mod echolocation;
pub mod wave;

pub fn echo_plugin(app: &mut App) {
    app.add_event::<EcholocationHitEv>()
        .add_event::<EcholocationEv>()
        .add_systems((echolocate, echo_hit).in_set(UnpausedGame))
        .add_system(flash_on_echolocation)
        .add_system(spawn_wave.run_if(resource_exists::<TextureAssets>()));
}
