use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::SpriteColorLens, *};

use crate::{
    animation::{
        get_fade_out_sprite_tween, SpriteRelativeColorLens, TweenDoneAction, UiBackgroundColorLens,
    },
    time::time::{ScaledTime, ScaledTimeDelta},
};

use super::AppState;

#[derive(Component)]
pub struct PersistReset;

#[derive(Component)]
pub struct FadeNode;

#[derive(Resource, Deref, DerefMut)]
pub struct FadeReset(Option<AppState>);

impl FadeReset {
    pub fn set(&mut self, state: AppState) {
        self.0 = Some(state);
    }
}

pub(crate) fn reset_plugin(app: &mut App) {
    app.insert_resource(FadeReset(None))
        .add_system(start_reset_fade_out.run_if(resource_changed::<FadeReset>()))
        .add_system(fade_in.run_if(state_changed::<AppState>()));
}

fn start_reset_fade_out(mut cmd: Commands, mut fade_reset: ResMut<FadeReset>) {
    if fade_reset.is_some() {
        cmd.spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                size: Size::all(Val::Px(5000.)),
                ..default()
            },
            ..default()
        })
        .insert(Animator::new(
            Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(500),
                UiBackgroundColorLens {
                    end: Color::BLACK,
                    ..default()
                },
            )
            .with_completed_event(TweenDoneAction::ResetAndNextState(
                fade_reset.take().unwrap(),
            )),
        ))
        .insert(FadeNode)
        .insert(PersistReset);
    }
}

fn fade_in(mut fade_q: Query<&mut Animator<BackgroundColor, TweenDoneAction>, With<FadeNode>>) {
    for mut anim in fade_q.iter_mut() {
        anim.set_tweenable(
            Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(500),
                UiBackgroundColorLens {
                    end: Color::NONE,
                    ..default()
                },
            )
            .with_completed_event(TweenDoneAction::DespawnRecursive),
        );
    }
}
