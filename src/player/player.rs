use crate::{
    agent::agent::{MovementDirection, Speed},
    assets::textures::TextureAssets,
    input::actions::{PlayerAction, UiAction},
    level::level::LevelEntry,
    time::time::{ScaledTime, ScaledTimeDelta},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

// todo: bind gamepads?
pub(super) fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    entry_q: Query<(), Added<LevelEntry>>,
) {
    if entry_q.iter().next().is_some() {
        commands
            .spawn(SpriteBundle {
                texture: textures.texture_bevy.clone(),
                transform: Transform::from_translation(Vec3::new(200., -60., 1.))
                    .with_scale(Vec2::splat(0.2).extend(1.)),
                sprite: Sprite {
                    color: Color::GRAY,
                    ..default()
                },
                ..Default::default()
            })
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::ball(100.))
            .insert(KinematicCharacterController {
                max_slope_climb_angle: 90f32.to_radians(),
                min_slope_slide_angle: 90f32.to_radians(),
                ..default()
            })
            .insert(Player)
            .insert(MovementDirection::default())
            .insert(Speed(100.))
            .insert(InputManagerBundle::<PlayerAction> {
                input_map: InputMap::default()
                    .insert(DualAxis::left_stick(), PlayerAction::Move)
                    .insert(VirtualDPad::wasd(), PlayerAction::Move)
                    .insert(VirtualDPad::arrow_keys(), PlayerAction::Move)
                    .insert(KeyCode::Escape, PlayerAction::Pause)
                    .insert(GamepadButtonType::Start, PlayerAction::Pause)
                    .insert(KeyCode::Space, PlayerAction::Echo)
                    .insert(GamepadButtonType::South, PlayerAction::Echo)
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
