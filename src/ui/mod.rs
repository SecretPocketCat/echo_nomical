use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use seldom_fn_plugin::FnPluginExt;

use crate::{
    animation::{get_relative_text_color_anim, get_relative_ui_bg_color_anim, TweenDoneAction},
    state::{AppState, FadeReset, GameState, PersistReset},
    AppSize, EntityCommandsExt,
};

use self::splash::show_splash;

mod button;
mod game_over;
mod menu;
mod pause;
mod splash;

pub fn ui_plugin(app: &mut App) {
    app.fn_plugin(button::button_plugin)
        .add_startup_system(setup_root_node)
        .add_system(resize_root_node.run_if(
            resource_exists::<RootUiNode>().and_then(resource_exists_and_changed::<AppSize>()),
        ))
        .add_system(menu::setup_ui.in_schedule(OnEnter(AppState::Menu)))
        .add_system(game_over::setup_ui.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(pause::setup_ui.in_schedule(OnEnter(GameState::Paused)))
        .add_system(teardown_ui.in_schedule(OnExit(GameState::Paused)))
        .add_system(show_splash.in_schedule(OnEnter(AppState::Splash)))
        .add_system(
            fade_to_menu
                .in_set(OnUpdate(AppState::Splash))
                .run_if(on_timer(Duration::from_secs(2)).and_then(run_once())),
        );
}

#[derive(Component)]
pub struct UiDisabled;

#[derive(Resource)]
pub struct RootUiNode(pub Entity);

fn setup_root_node(mut cmd: Commands, size: Res<AppSize>) {
    let root = cmd
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(size.x), Val::Px(size.y)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(PersistReset)
        .id();

    cmd.insert_resource(RootUiNode(root));
}

fn resize_root_node(size: Res<AppSize>, root: Res<RootUiNode>, mut style_q: Query<&mut Style>) {
    let mut style = style_q
        .get_mut(root.0)
        .expect("Root node should always exist");
    style.size = Size::new(Val::Px(size.x), Val::Px(size.y));
}

fn teardown_ui(
    mut cmd: Commands,
    teardown_bg_q: Query<Entity, (With<BackgroundColor>, Without<PersistReset>)>,
    teardown_txt_q: Query<Entity, (With<BackgroundColor>, Without<PersistReset>)>,
) {
    for e in teardown_bg_q.iter() {
        cmd.entity(e)
            .try_insert(get_relative_ui_bg_color_anim(
                Color::NONE,
                350,
                TweenDoneAction::DespawnSelfRecursive,
            ))
            .try_insert(UiDisabled);
    }

    for e in teardown_txt_q.iter() {
        cmd.entity(e)
            .try_insert(get_relative_text_color_anim(
                Color::NONE,
                350,
                TweenDoneAction::DespawnSelfRecursive,
            ))
            .try_insert(UiDisabled);
    }
}

fn fade_to_menu(mut fade_reset: ResMut<FadeReset>) {
    fade_reset.set(AppState::Menu);
}
