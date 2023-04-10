use crate::{
    agent::agent::{MovementDirection, MovementDirectionEasing, Speed},
    animation::{
        delay_tween, get_relative_scale_anim, get_relative_sprite_color_anim, get_scale_anim,
        get_scale_tween, TweenDoneAction,
    },
    assets::textures::TextureAssets,
    echolocation::{echolocation::EcholocationHitColor, wave::Wave},
    enemy::EnemyType,
    input::actions::{PlayerAction, UiAction},
    level::level::{LevelEntry, LevelExit},
    physics::{check_collision_start_pair, ECHO_COLL_GROUP, PLAYER_COLL_GROUP},
    state::{AppState, FadeReset},
    EntityCommandsExt,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{Animator, EaseFunction};
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
    entry_q: Query<&Transform, Added<LevelEntry>>,
) {
    let radius = 20.;

    if let Some(transform) = entry_q.iter().next() {
        cmd.spawn(SpatialBundle::from_transform(*transform))
            .insert(RigidBody::KinematicPositionBased)
            .insert(Collider::ball(radius * 0.8))
            .insert(KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                max_slope_climb_angle: 90f32.to_radians(),
                min_slope_slide_angle: 0f32.to_radians(),
                ..default()
            })
            .insert(CollisionGroups::new(
                PLAYER_COLL_GROUP.into(),
                Group::all()
                    .difference(ECHO_COLL_GROUP)
                    .difference(PLAYER_COLL_GROUP),
            ))
            .insert(Player)
            .insert(MovementDirection::default())
            .insert(MovementDirectionEasing::new(0.085))
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
            })
            .insert(Name::new("PLAYER"))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        texture: textures.player.clone(),
                        transform: Transform::from_scale(Vec2::ZERO.extend(0.)),
                        sprite: Sprite {
                            color: Color::SEA_GREEN,
                            custom_size: Some(Vec2::splat(radius * 2.)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Animator::new(delay_tween(
                        get_scale_tween(
                            None,
                            Vec3::ONE,
                            400,
                            EaseFunction::BackOut,
                            TweenDoneAction::None,
                        ),
                        500,
                    )));
            });
    }
}

pub(super) fn move_player(
    mut player_q: Query<(&mut MovementDirection, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut dir, actions) in &mut player_q {
        if let Some(movement) = actions.clamped_axis_pair(PlayerAction::Move) {
            dir.0 = movement.xy().normalize_or_zero();
        }
    }
}

pub(super) fn exit_reached(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_q: Query<(), With<Player>>,
    exit_q: Query<(), With<LevelExit>>,
    col_q: Query<(&EcholocationHitColor, &GlobalTransform)>,
    mut fade_reset: ResMut<FadeReset>,
    mut ev_w: EventWriter<PlayerEv>,
) {
    if let Some(coll) = collision_events
        .iter()
        .filter_map(|ev| check_collision_start_pair(ev, &player_q, &exit_q))
        .next()
    {
        fade_reset.set(AppState::Game);
        ev_w.send(PlayerEv::ClearedLevel);

        cmd.entity(coll.0).try_insert(get_scale_anim(
            None,
            Vec2::ZERO.extend(1.),
            400,
            EaseFunction::BackIn,
            TweenDoneAction::DespawnSelfRecursive,
        ));

        if let Ok((col, t)) = col_q.get(coll.1) {
            cmd.entity(coll.1)
                .try_insert(get_scale_anim(
                    None,
                    Vec2::ZERO.extend(1.),
                    400,
                    EaseFunction::BackIn,
                    TweenDoneAction::DespawnSelfRecursive,
                ))
                .try_insert(get_relative_sprite_color_anim(
                    col.0,
                    200,
                    TweenDoneAction::None,
                ));

            cmd.spawn(Wave {
                position: t.translation(),
                radius: 130.,
                color: col.0,
            });
        }
    }
}

pub(super) fn player_hit(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_q: Query<(), With<Player>>,
    enemy_q: Query<(), With<EnemyType>>,
    trans_q: Query<&GlobalTransform>,
    mut fade_reset: ResMut<FadeReset>,
    mut ev_w: EventWriter<PlayerEv>,
) {
    if let Some(coll) = collision_events
        .iter()
        .filter_map(|ev| check_collision_start_pair(ev, &player_q, &enemy_q))
        .next()
    {
        fade_reset.set(AppState::GameOver);
        ev_w.send(PlayerEv::Died);

        if let Ok(t) = trans_q.get(coll.0) {
            cmd.spawn(Wave {
                position: t.translation(),
                radius: 130.,
                color: Color::SEA_GREEN,
            });

            cmd.entity(coll.0).try_insert(get_relative_scale_anim(
                Vec2::ZERO.extend(1.),
                300,
                TweenDoneAction::DespawnSelfRecursive,
            ));
        }
    }
}
