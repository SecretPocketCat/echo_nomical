use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

#[repr(u64)]
#[derive(Clone)]
pub enum TweenDoneAction {
    None = 0,
    DespawnRecursive = 1,
}

impl From<u64> for TweenDoneAction {
    fn from(val: u64) -> Self {
        unsafe { ::std::mem::transmute(val) }
    }
}

impl From<TweenDoneAction> for u64 {
    fn from(val: TweenDoneAction) -> Self {
        val as u64
    }
}

pub fn on_tween_completed(
    mut commands: Commands,
    mut ev_reader: EventReader<TweenCompleted>,
    entity_q: Query<Entity>,
) {
    for ev in ev_reader.iter() {
        match TweenDoneAction::from(ev.user_data) {
            TweenDoneAction::None => {}
            TweenDoneAction::DespawnRecursive => {
                if entity_q.get(ev.entity).is_ok() {
                    commands.entity(ev.entity).despawn_recursive();
                }
            }
        }
    }
}
