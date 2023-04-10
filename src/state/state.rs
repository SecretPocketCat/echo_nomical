use bevy::{app::AppExit, prelude::*};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // #[default]
    // Splash,
    #[default]
    Loading,
    Menu,
    Game,
    GameOver,
    Tutorial,
    Quit,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Fading,
}

pub fn reset_state<T: States>(mut state: ResMut<NextState<T>>) {
    state.set(T::default())
}

pub fn set_state_fn<S: States>(next_state: S) -> impl FnMut(ResMut<NextState<S>>) {
    move |mut state: ResMut<NextState<S>>| {
        state.set(next_state.clone());
    }
}

pub(super) fn quit_app(mut exit_ev_w: EventWriter<AppExit>) {
    exit_ev_w.send(AppExit);
}
