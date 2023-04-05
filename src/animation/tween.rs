#![allow(dead_code)]

use bevy::prelude::*;
use bevy_tweening::lens::{SpriteColorLens, TransformPositionLens, TransformScaleLens};
use bevy_tweening::*;
use std::time::Duration;

use super::tween_events::TweenDoneAction;

pub fn delay_tween<T: 'static>(tween: Tween<T>, delay_ms: u64) -> Sequence<T> {
    if delay_ms > 0 {
        Delay::new(Duration::from_millis(delay_ms)).then(tween)
    } else {
        Sequence::new([tween])
    }
}

pub fn get_relative_move_anim(
    end_pos: Vec3,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Transform> {
    Animator::new(get_relative_move_tween(end_pos, duration_ms, on_completed))
}

pub fn get_relative_move_tween(
    end_pos: Vec3,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Transform> {
    let mut tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration_ms),
        TransformRelativePositionLens {
            start: Vec3::ZERO,
            end: end_pos,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_move_anim(
    start_pos: Vec3,
    end_pos: Vec3,
    duration_ms: u64,
    ease: EaseFunction,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Transform> {
    Animator::new(get_move_tween(
        start_pos,
        end_pos,
        duration_ms,
        ease,
        on_completed,
    ))
}

pub fn get_relative_move_by_anim(
    move_by: Vec3,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Transform> {
    Animator::new(get_relative_move_by_tween(
        move_by,
        duration_ms,
        EaseFunction::QuadraticInOut,
        on_completed,
    ))
}

pub fn get_relative_move_by_tween(
    move_by: Vec3,
    duration_ms: u64,
    ease: EaseFunction,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Transform> {
    let mut tween = Tween::new(
        ease,
        Duration::from_millis(duration_ms),
        TransformRelativeByPositionLens::new(move_by),
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_move_tween(
    start_pos: Vec3,
    end_pos: Vec3,
    duration_ms: u64,
    ease: EaseFunction,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Transform> {
    let mut tween = Tween::new(
        ease,
        Duration::from_millis(duration_ms),
        TransformPositionLens {
            start: start_pos,
            end: end_pos,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_relative_sprite_color_anim(
    col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Sprite> {
    Animator::new(get_relative_sprite_color_tween(
        col,
        duration_ms,
        on_completed,
    ))
}

pub fn get_relative_sprite_color_tween(
    col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Sprite> {
    let mut tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration_ms),
        SpriteRelativeColorLens {
            start: Color::NONE,
            end: col,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_relative_fade_text_anim(
    col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Text> {
    Animator::new(get_relative_fade_text_tween(
        col,
        duration_ms,
        EaseFunction::QuadraticInOut,
        on_completed,
    ))
}

pub fn get_relative_fade_text_tween(
    col: Color,
    duration_ms: u64,
    ease: EaseFunction,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Text> {
    let mut tween = Tween::new(
        ease,
        Duration::from_millis(duration_ms),
        TextRelativeColorLens {
            start: Vec::new(),
            end: col,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_relative_spritesheet_color_anim(
    col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<TextureAtlasSprite> {
    Animator::new(get_relative_fade_spritesheet_tween(
        col,
        duration_ms,
        on_completed,
    ))
}

pub fn get_relative_fade_spritesheet_tween(
    col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Tween<TextureAtlasSprite> {
    let mut tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration_ms),
        SpriteSheetRelativeColorLens {
            start: Color::NONE,
            end: col,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_scale_tween(
    start_scale: Vec3,
    end_scale: Vec3,
    ease: EaseFunction,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Transform> {
    let mut tween = Tween::new(
        ease,
        Duration::from_millis(duration_ms),
        TransformScaleLens {
            start: start_scale,
            end: end_scale,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

pub fn get_fade_out_sprite_anim(
    start_col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Animator<Sprite> {
    Animator::new(get_fade_out_sprite_tween(
        start_col,
        duration_ms,
        on_completed,
    ))
}

pub fn get_fade_out_sprite_tween(
    start_col: Color,
    duration_ms: u64,
    on_completed: Option<TweenDoneAction>,
) -> Tween<Sprite> {
    let mut tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration_ms),
        SpriteColorLens {
            start: start_col,
            end: Color::NONE,
        },
    );

    if let Some(on_completed) = on_completed {
        tween = tween.with_completed_event(on_completed.into());
    }

    tween
}

#[derive(Default)]
pub struct SpriteRelativeColorLens {
    start: Color,
    pub end: Color,
}

impl Lens<Sprite> for SpriteRelativeColorLens {
    fn lerp(&mut self, target: &mut Sprite, ratio: f32) {
        target.color = lerp_color(self.start, self.end, ratio);
    }

    fn update_on_tween_start(&mut self, target: &Sprite) {
        self.start = target.color;
    }
}

#[derive(Default)]
pub struct SpriteSheetRelativeColorLens {
    start: Color,
    pub end: Color,
}

impl Lens<TextureAtlasSprite> for SpriteSheetRelativeColorLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        target.color = lerp_color(self.start, self.end, ratio);
    }

    fn update_on_tween_start(&mut self, target: &TextureAtlasSprite) {
        self.start = target.color;
    }
}

#[derive(Default)]
pub struct TransformRelativeScaleLens {
    pub start: Vec3,
    pub end: Vec3,
}

impl Lens<Transform> for TransformRelativeScaleLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.scale = value;
    }

    fn update_on_tween_start(&mut self, target: &Transform) {
        self.start = target.scale;
    }
}

#[derive(Default)]
pub struct TransformRelativePositionLens {
    start: Vec3,
    pub end: Vec3,
}

impl Lens<Transform> for TransformRelativePositionLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
    }

    fn update_on_tween_start(&mut self, target: &Transform) {
        self.start = target.translation;
    }
}

#[derive(Default)]
pub struct TransformRelativeByPositionLens {
    start: Vec3,
    end: Vec3,
    pub move_by: Vec3,
}

impl TransformRelativeByPositionLens {
    pub fn new(move_by: Vec3) -> Self {
        Self {
            move_by,
            start: Vec3::ZERO,
            end: Vec3::ZERO,
        }
    }
}

impl Lens<Transform> for TransformRelativeByPositionLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
    }

    fn update_on_tween_start(&mut self, target: &Transform) {
        self.start = target.translation;
        self.end = target.translation + self.move_by;
    }
}

#[derive(Default)]
pub struct TextRelativeColorLens {
    start: Vec<Color>,
    pub end: Color,
}

impl Lens<Text> for TextRelativeColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        for i in 0..target.sections.len() {
            target.sections[i].style.color = lerp_color(self.start[i], self.end, ratio);
        }
    }

    fn update_on_tween_start(&mut self, target: &Text) {
        self.start = target.sections.iter().map(|s| s.style.color).collect();
    }
}

fn lerp_color(from: Color, to: Color, ratio: f32) -> Color {
    let start: Vec4 = from.into();
    let end: Vec4 = to.into();
    start.lerp(end, ratio).into()
}
