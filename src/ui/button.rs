use crate::{
    assets::fonts::FontAssets,
    state::{AppState, FadeReset},
};
use bevy::prelude::*;

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
}

#[derive(Component)]
pub(super) struct UiButton {
    pub text: String,
    pub action: UiButtonAction,
    pub primary: bool,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
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
            .insert(ButtonBundle {
                style: Style {
                    padding: UiRect::new(
                        Val::Px(pad_x),
                        Val::Px(pad_x),
                        Val::Px(pad_y),
                        Val::Px(pad_y),
                    ),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(25.)),
                    ..default()
                },
                background_color: button_colors.normal.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    ui_btn.text.clone(),
                    TextStyle {
                        font: font_assets.menu.clone(),
                        font_size: if ui_btn.primary { 60.0 } else { 40. },
                        color: Color::ANTIQUE_WHITE,
                    },
                ));
            });
    }
}

fn on_ui_btn_interaction(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UiButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut fade_reset: ResMut<FadeReset>,
) {
    for (interaction, mut color, ui_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match &ui_btn.action {
                UiButtonAction::ChangeState(next) => fade_reset.set(next.clone()),
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
