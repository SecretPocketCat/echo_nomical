use std::time::Duration;

use bevy::input::mouse::MouseButtonInput;
use bevy::{ecs::system::Resource, prelude::*};
use bevy_kira_audio::prelude::*;
use rand::seq::SliceRandom;
use rand::*;

use crate::assets::audio::SfxAssets;
use crate::echolocation::echolocation::EcholocationEv;
use crate::enemy::EnemyEv;
use crate::io::save::VolumeSettings;
use crate::player::player::PlayerEv;
use crate::state::AppState;

// todo: use sfx + master volume
pub(super) fn sfx_plugin(app: &mut bevy::prelude::App) {
    app.add_audio_channel::<SfxChannel>()
        .add_systems(
            (
                play_sfx_on_evt::<EcholocationEv>,
                play_sfx_on_evt::<PlayerEv>,
                play_sfx_on_evt::<EnemyEv>,
            )
                .in_base_set(CoreSet::PostUpdate)
                .distributive_run_if(resource_exists::<SfxAssets>()),
        )
        .add_system(set_sfx_volume.run_if(resource_changed::<VolumeSettings>()))
        .add_system(play_drone.in_schedule(OnExit(AppState::Loading)));
}

#[derive(Resource)]
struct SfxChannel;

pub trait SfxEv {
    fn get_volume(&self) -> f64;
    fn get_sfx_handles(&self, sfx: &SfxAssets) -> Vec<Handle<AudioSource>>;
    fn skip(&self) -> bool {
        false
    }
}

impl SfxEv for EcholocationEv {
    fn get_volume(&self) -> f64 {
        1.0
    }

    fn get_sfx_handles(&self, sfx: &SfxAssets) -> Vec<Handle<AudioSource>> {
        sfx.echo.clone()
    }
}

impl SfxEv for PlayerEv {
    fn get_volume(&self) -> f64 {
        match self {
            PlayerEv::ClearedLevel => 0.5,
            PlayerEv::Died => 0.9,
        }
    }

    fn get_sfx_handles(&self, sfx: &SfxAssets) -> Vec<Handle<AudioSource>> {
        match self {
            PlayerEv::ClearedLevel => vec![sfx.level_cleared.clone()],
            PlayerEv::Died => sfx.player_death.clone(),
        }
    }
}

impl SfxEv for EnemyEv {
    fn get_volume(&self) -> f64 {
        match self {
            EnemyEv::Killed => 0.85,
            EnemyEv::Alarmed => 0.3,
        }
    }

    fn get_sfx_handles(&self, sfx: &SfxAssets) -> Vec<Handle<AudioSource>> {
        match self {
            EnemyEv::Killed => sfx.enemy_death.clone(),
            EnemyEv::Alarmed => sfx.enemy_alert.clone(),
        }
    }
}

fn play_sfx_on_evt<TEvt: Event + SfxEv>(
    mut ev_r: EventReader<TEvt>,
    audio: Res<AudioChannel<SfxChannel>>,
    sfx: Res<SfxAssets>,
    volume: Res<VolumeSettings>,
) {
    let mut rand = thread_rng();

    for ev in ev_r.iter() {
        if ev.skip() {
            continue;
        }

        if let Some(sfx_handle) = ev.get_sfx_handles(&sfx).choose(&mut rand) {
            audio
                .play(sfx_handle.clone())
                .with_volume(ev.get_volume() * volume.get_sfx_volume());
        }
    }
}

fn set_sfx_volume(audio: Res<AudioChannel<SfxChannel>>, volume: Res<VolumeSettings>) {
    audio.set_volume(volume.get_sfx_volume());
}

fn play_drone(audio: Res<Audio>, assets: Res<SfxAssets>) {
    audio
        .play(assets.drone.clone())
        .loop_from(1.5)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .with_volume(0.3);
}
