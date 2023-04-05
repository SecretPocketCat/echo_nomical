use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{
    state::{reset_state, set_state_fn},
    AppState, GameState,
};
use crate::input::actions::{any_player_just_released, PlayerAction, UiAction};

pub(crate) fn pause_plugin(app: &mut App) {
    app.add_system(toggle_input::<PlayerAction, false>.in_schedule(OnEnter(GameState::Paused)))
        .add_system(toggle_input::<PlayerAction, true>.in_schedule(OnExit(GameState::Paused)))
        .add_system(toggle_input::<PlayerAction, true>.in_schedule(OnEnter(AppState::Game)))
        .add_system(reset_state::<GameState>.run_if(state_changed::<AppState>()))
        .add_system(
            set_state_fn(GameState::Paused)
                .in_set(OnUpdate(AppState::Game))
                .run_if(
                    in_state(GameState::Running)
                        .and_then(any_player_just_released(PlayerAction::Pause)),
                ),
        )
        .add_system(
            set_state_fn(GameState::Running)
                .in_set(OnUpdate(AppState::Game))
                .run_if(
                    in_state(GameState::Paused)
                        .and_then(any_player_just_released(UiAction::Cancel)),
                ),
        );
}

fn toggle_input<T: Actionlike, const ENABLE: bool>(
    mut cmd: Commands,
    toggle: Option<ResMut<ToggleActions<T>>>,
) {
    if let Some(mut toggle) = toggle {
        toggle.enabled = ENABLE;
    } else {
        let mut toggle = ToggleActions::<T>::default();
        toggle.enabled = ENABLE;
        cmd.insert_resource(toggle);
    }
}
