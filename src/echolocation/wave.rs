use bevy::prelude::*;

use bevy_tweening::{Animator, EaseFunction};


use crate::{
    animation::{
        get_relative_sprite_color_tween, get_scale_anim, TweenDoneAction,
    },
    assets::textures::TextureAssets,
    EntityCommandsExt,
};

#[derive(Component)]
pub struct Wave {
    pub position: Vec3,
    pub radius: f32,
    pub color: Color,
}

pub(super) fn spawn_wave(
    mut cmd: Commands,
    wave_q: Query<(Entity, &Wave), Added<Wave>>,
    textures: Res<TextureAssets>,
) {
    for (e, wave) in wave_q.iter() {
        cmd.entity(e)
            .try_insert(SpriteBundle {
                transform: Transform::from_scale(Vec2::ZERO.extend(1.))
                    .with_translation(wave.position),
                texture: textures.wave.clone(),
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::splat(wave.radius * 2.)),
                    ..default()
                },
                ..Default::default()
            })
            .try_insert(get_scale_anim(
                None,
                Vec3::ONE,
                1500,
                EaseFunction::BackOut,
                TweenDoneAction::None,
            ))
            .try_insert(Animator::new(
                get_relative_sprite_color_tween(wave.color, 150, TweenDoneAction::None).then(
                    get_relative_sprite_color_tween(
                        Color::NONE,
                        1400,
                        TweenDoneAction::DespawnSelfRecursive,
                    ),
                ),
            ))
            .try_insert(Name::new("Wave"));
    }
}
