use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn actions_plugin(app: &mut App) {
    app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(InputManagerPlugin::<UiAction>::default());
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    Move,
    Echo,
    Pause,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum UiAction {
    Move,
    Confirm,
    Cancel,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum DebugAction {
    ToggleRapierDebug,
    RestartGame,
}

pub fn any_player_just_released<A: Actionlike>(
    action: A,
) -> impl FnMut(Query<&ActionState<A>>) -> bool {
    move |input_q: Query<&ActionState<A>>| {
        input_q
            .iter()
            .find(|input| input.just_released(action.clone()))
            .is_some()
    }
}
