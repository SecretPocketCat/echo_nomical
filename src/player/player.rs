use crate::{
    agent::agent::{MovementDirection, Speed},
    assets::textures::TextureAssets,
    enemy::Enemy,
    input::actions::{PlayerAction, UiAction},
    level::level::{LevelEntry, LevelExit},
    physics::check_collision_start_pair,
    state::{AppState, FadeReset},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerEv {
    ClearedLevel,
    Died,
}

// todo: bind gamepads?
pub(super) fn spawn_player(
    mut cmd: Commands,
    textures: Res<TextureAssets>,
    entry_q: Query<(), Added<LevelEntry>>,
) {
    if entry_q.iter().next().is_some() {
        cmd.spawn(SpriteBundle {
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
            filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
            max_slope_climb_angle: 90f32.to_radians(),
            min_slope_slide_angle: 0f32.to_radians(),
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

pub(super) fn exit_reached(
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<(), With<Player>>,
    q_exit: Query<(), With<LevelExit>>,
    mut fade_reset: ResMut<FadeReset>,
    mut ev_w: EventWriter<PlayerEv>,
) {
    if let Some(..) = collision_events
        .iter()
        .filter(|ev| check_collision_start_pair(ev, &q_player, &q_exit))
        .next()
    {
        fade_reset.set(AppState::Game);
        ev_w.send(PlayerEv::ClearedLevel);
    }
}

pub(super) fn player_hit(
    mut collision_events: EventReader<CollisionEvent>,
    q_player: Query<(), With<Player>>,
    q_enemy: Query<(), With<Enemy>>,
    mut fade_reset: ResMut<FadeReset>,
    mut ev_w: EventWriter<PlayerEv>,
) {
    if let Some(..) = collision_events
        .iter()
        .filter(|ev| check_collision_start_pair(ev, &q_player, &q_enemy))
        .next()
    {
        fade_reset.set(AppState::GameOver);
        ev_w.send(PlayerEv::Died);
    }
}
