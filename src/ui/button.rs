use crate::{
    animation::{get_relative_text_color_anim, get_relative_ui_bg_color_anim, TweenDoneAction},
    assets::fonts::FontAssets,
    palette::{COL_PLAYER, COL_PORTAL, COL_TEXT},
    state::{AppState, FadeReset, GameState},
    EntityCommandsExt,
};
use bevy::prelude::*;

use super::UiDisabled;

pub(super) fn button_plugin(app: &mut App) {
    app.init_resource::<ButtonColors>()
        .add_system(spawn_ui_btn.run_if(resource_exists::<FontAssets>()))
        .add_system(on_ui_btn_interaction);
}

#[derive(Resource)]
pub(super) struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

pub(super) enum UiButtonAction {
    ChangeState(AppState),
    Unpause,
}

#[derive(Component)]
pub(super) struct UiButton {
    pub text: String,
    pub action: UiButtonAction,
    pub primary: bool,
    pub margin: Option<UiRect>,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: COL_PORTAL,
            hovered: COL_PLAYER,
        }
    }
}

fn spawn_ui_btn(
    mut cmd: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    spawn_q: Query<(Entity, &UiButton), Added<UiButton>>,
) {
    for (e, ui_btn) in spawn_q.iter() {
        let pad_x = if ui_btn.primary { 35.0 } else { 25. };
        let pad_y = if ui_btn.primary { 20.0 } else { 10. };

        cmd.entity(e)
            .try_insert(ButtonBundle {
                style: Style {
                    padding: UiRect::new(
                        Val::Px(pad_x),
                        Val::Px(pad_x),
                        Val::Px(pad_y),
                        Val::Px(pad_y),
                    ),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: ui_btn
                        .margin
                        .unwrap_or(UiRect::vertical(Val::Px(if ui_btn.primary {
                            20.0
                        } else {
                            12.
                        }))),
                    min_size: Size::width(Val::Percent(if ui_btn.primary { 40.0 } else { 25. })),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            })
            .try_insert(get_relative_ui_bg_color_anim(
                button_colors.normal,
                350,
                TweenDoneAction::None,
            ))
            .with_children(|parent| {
                parent
                    .spawn(TextBundle::from_section(
                        ui_btn.text.clone(),
                        TextStyle {
                            font: font_assets.menu.clone(),
                            font_size: if ui_btn.primary { 60.0 } else { 40. },
                            color: Color::NONE,
                        },
                    ))
                    .insert(get_relative_text_color_anim(
                        COL_TEXT,
                        500,
                        TweenDoneAction::None,
                    ));
            });
    }
}

fn on_ui_btn_interaction(
    mut cmd: Commands,
    button_colors: Res<ButtonColors>,
    interaction_query: Query<
        (Entity, &Interaction, &UiButton),
        (Changed<Interaction>, With<Button>, Without<UiDisabled>),
    >,
    mut fade_reset: ResMut<FadeReset>,
    mut next_pause_state: ResMut<NextState<GameState>>,
) {
    for (e, interaction, ui_btn) in interaction_query.iter() {
        if let Some(col) = match *interaction {
            Interaction::Clicked => {
                match &ui_btn.action {
                    UiButtonAction::ChangeState(next) => fade_reset.set(next.clone()),
                    UiButtonAction::Unpause => next_pause_state.set(GameState::Running),
                };

                None
            }
            Interaction::Hovered => Some(button_colors.hovered),
            Interaction::None => Some(button_colors.normal),
        } {
            // todo: maybe scale it a bit too
            cmd.entity(e).insert(get_relative_ui_bg_color_anim(
                col,
                175,
                TweenDoneAction::None,
            ));
        }
    }
}
