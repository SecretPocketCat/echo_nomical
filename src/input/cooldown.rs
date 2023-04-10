use std::marker::PhantomData;

use bevy::prelude::*;

use crate::time::time::{ScaledTime, ScaledTimeDelta};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Cooldown<T: Send + Sync + 'static> {
    timer: Timer,
    _phantom: PhantomData<T>,
}

impl<T: Send + Sync> Cooldown<T> {
    pub fn new(duration_s: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_s, TimerMode::Once),
            _phantom: PhantomData::default(),
        }
    }
}

pub fn process_cooldown<T: Send + Sync>(
    mut cmd: Commands,
    mut cooldown_q: Query<(Entity, &mut Cooldown<T>)>,
    time: ScaledTime,
) {
    for (e, mut cooldown) in cooldown_q.iter_mut() {
        cooldown.timer.tick(time.scaled_delta());

        if cooldown.timer.just_finished() {
            cmd.entity(e).remove::<Cooldown<T>>();
        }
    }
}
