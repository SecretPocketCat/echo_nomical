use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::state::{AppState, PersistReset};

#[repr(u64)]
#[derive(Clone)]
pub enum TweenDoneAction {
    None = 0,
    DespawnRecursive = 1,
    ResetAndNextState(AppState),
}

pub fn on_tween_completed(
    mut cmd: Commands,
    mut ev_reader: EventReader<TweenCompleted<TweenDoneAction>>,
    entity_q: Query<Entity>,
    reset_q: Query<
        Entity,
        (
            Without<PersistReset>,
            Without<Window>,
            Without<Camera>,
            Without<DebugLinesMesh>,
        ),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ev in ev_reader.iter() {
        warn!("tween done");
        match &ev.user_data {
            TweenDoneAction::None => {}
            TweenDoneAction::DespawnRecursive => {
                if entity_q.get(ev.entity).is_ok() {
                    cmd.entity(ev.entity).despawn_recursive();
                }
            }
            TweenDoneAction::ResetAndNextState(next) => {
                for e in reset_q.iter() {
                    cmd.entity(e).despawn_recursive();
                }

                next_state.set(next.clone());
            }
        }
    }
}
