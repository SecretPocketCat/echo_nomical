use bevy::prelude::*;
use bevy_tweening::*;

use crate::{
    animation::{get_relative_ui_bg_color_anim, get_relative_ui_bg_color_tween, TweenDoneAction},
    level::level::ReachedLevel,
    ui::RootUiNode,
};

use super::{AppState, GameState};

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
        .add_system(fade_in.run_if(state_changed::<AppState>()))
        .add_system(
            reset_resource_to_default::<ReachedLevel>.in_schedule(OnExit(AppState::GameOver)),
        );
}

fn start_reset_fade_out(
    mut cmd: Commands,
    mut fade_reset: ResMut<FadeReset>,
    mut next_state: ResMut<NextState<GameState>>,
    root: Res<RootUiNode>,
) {
    if fade_reset.is_some() {
        next_state.set(GameState::Fading);

        cmd.entity(root.0).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::all(Val::Percent(110.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(ZIndex::Global(10000))
                .insert(get_relative_ui_bg_color_anim(
                    Color::BLACK,
                    500,
                    TweenDoneAction::ResetAndNextState(fade_reset.take().unwrap()),
                ))
                .insert(FadeNode)
                .insert(PersistReset);
        });
    }
}

fn fade_in(mut fade_q: Query<&mut Animator<BackgroundColor, TweenDoneAction>, With<FadeNode>>) {
    for mut anim in fade_q.iter_mut() {
        anim.set_tweenable(get_relative_ui_bg_color_tween(
            Color::NONE,
            500,
            TweenDoneAction::DespawnSelfRecursive,
        ));
    }
}

pub fn reset_resource_to_default<T: Resource + Default>(mut state: ResMut<T>) {
    *state = T::default();
}
