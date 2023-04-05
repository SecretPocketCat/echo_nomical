use bevy::{prelude::*, window::WindowCloseRequested};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::{state::AppState, GAME_NAME};

const SETTINGS_KEY: &str = "game_save";

pub(super) fn save_plugin(app: &mut App) {
    app.insert_resource(PkvStore::new("SecretPocketCat", GAME_NAME))
        .add_system(
            save_game
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_changed::<AppState>().or_else(on_event::<WindowCloseRequested>())),
        )
        .add_startup_system(load_game);
}

#[derive(Serialize, Deserialize, Resource, Clone)]
pub struct VolumeSettings {
    master: f64,
    sfx: f64,
    music: f64,
    muted: bool,
}

impl VolumeSettings {
    pub fn get_sfx_volume(&self) -> f64 {
        self.master * self.sfx * self.muted_f64()
    }

    pub fn get_music_volume(&self) -> f64 {
        self.master * self.music * self.muted_f64()
    }

    fn muted_f64(&self) -> f64 {
        if self.muted {
            0.
        } else {
            1.
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameSettings {
    volume: VolumeSettings,
}

fn save_game(mut pkv: ResMut<PkvStore>, volume_set: Res<VolumeSettings>) {
    pkv.set(
        SETTINGS_KEY,
        &GameSettings {
            volume: volume_set.clone(),
        },
    )
    .expect("failed to store settings");
}

fn load_game(pkv: Res<PkvStore>, mut cmd: Commands) {
    let settings = pkv
        .get::<GameSettings>(SETTINGS_KEY)
        .unwrap_or_else(|_| GameSettings {
            volume: VolumeSettings {
                master: 0.5,
                sfx: 0.5,
                music: 0.5,
                muted: false,
            },
        });

    cmd.insert_resource(settings.volume);
}
