use crate::{assets::fonts::FontAssets, state::AppState};
use bevy::prelude::*;

use super::RootUiNode;

pub(super) fn menu_plugin(app: &mut App) {
    app.init_resource::<ButtonColors>()
        .add_system(setup_ui.in_schedule(OnEnter(AppState::Menu)))
        .add_system(click_play_button.in_set(OnUpdate(AppState::Menu)));
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    root: Res<RootUiNode>,
) {
    commands.entity(root.0).with_children(|parent| {
        parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: button_colors.normal.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font: font_assets.menu.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
