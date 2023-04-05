use crate::{
    agent::agent::{MovementDirection, Speed, Wrap},
    assets::textures::TextureAssets,
    input::actions::{PlayerAction, UiAction},
    time::time::{ScaledTime, ScaledTimeDelta},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

// todo: bind gamepads?
pub(super) fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec2::splat(0.25).extend(1.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(MovementDirection::default())
        .insert(Speed(400.))
        .insert(Wrap)
        .insert(InputManagerBundle::<PlayerAction> {
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), PlayerAction::Move)
                .insert(VirtualDPad::wasd(), PlayerAction::Move)
                .insert(VirtualDPad::arrow_keys(), PlayerAction::Move)
                .insert(KeyCode::Escape, PlayerAction::Pause)
                .insert(GamepadButtonType::Start, PlayerAction::Pause)
                .build(),
            ..default()
        })
        // this should maybe be added to ui, not the player?
        .insert(InputManagerBundle::<UiAction> {
            input_map: InputMap::default()
                .insert(KeyCode::Escape, UiAction::Cancel)
                .insert(GamepadButtonType::East, UiAction::Cancel)
                .insert(KeyCode::Return, UiAction::Confirm)
                .insert(KeyCode::Space, UiAction::Confirm)
                .insert(GamepadButtonType::South, UiAction::Confirm)
                .build(),
            ..default()
        });
}

pub(super) fn move_player(
    mut player_q: Query<(&mut MovementDirection, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut dir, actions) in &mut player_q {
        if let Some(movement) = actions.clamped_axis_pair(PlayerAction::Move) {
            dir.0 = movement.xy();
        }
    }
}
