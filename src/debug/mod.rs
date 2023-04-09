use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

use crate::{
    input::actions::DebugAction,
    state::{AppState, FadeReset},
};

pub fn debug_plugin(app: &mut App) {
    #[cfg(debug_assertions)]
    {
        app.add_plugin(EditorPlugin::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(InputManagerPlugin::<DebugAction>::default())
            .init_resource::<ActionState<DebugAction>>()
            .insert_resource(
                InputMap::default()
                    .insert(KeyCode::R, DebugAction::RestartGame)
                    .insert(KeyCode::C, DebugAction::ToggleRapierDebug)
                    .build(),
            )
            .add_system(
                toggle_rapier_debug.run_if(action_just_pressed(DebugAction::ToggleRapierDebug)),
            )
            .add_system(reset_game.run_if(action_just_pressed(DebugAction::RestartGame)));
    }
}

fn toggle_rapier_debug(mut dbg_ctx: ResMut<DebugRenderContext>) {
    dbg_ctx.enabled = !dbg_ctx.enabled;
}

fn reset_game(mut fade_reset: ResMut<FadeReset>) {
    fade_reset.set(AppState::Game);
}
