use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_tweening::TweenCompleted;

use crate::state::{AppState, PersistReset};

#[derive(Clone)]
pub enum TweenDoneAction {
    #[allow(dead_code)]
    None,
    DespawnRecursive,
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
            // this is just to not reset rapier debug as there's no actual usable marker
            Without<NoFrustumCulling>,
        ),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ev in ev_reader.iter() {
        warn!("tween done");
        match &ev.user_data {
            TweenDoneAction::None => {}
            TweenDoneAction::DespawnRecursive => {
                if entity_q.contains(ev.entity) {
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
