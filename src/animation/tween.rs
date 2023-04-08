#![allow(dead_code)]

use bevy::prelude::*;
use bevy_tweening::*;
use std::time::Duration;

use super::tween_events::TweenDoneAction;
use super::tween_lenses::*;
use super::tween_macros::*;

pub fn delay_tween<T: 'static>(
    tween: Tween<T, TweenDoneAction>,
    delay_ms: u64,
) -> Sequence<T, TweenDoneAction> {
    if delay_ms > 0 {
        Delay::new(Duration::from_millis(delay_ms)).then(tween)
    } else {
        Sequence::new([tween])
    }
}

relative_tween_fns!(
    translation,
    Transform,
    TweenDoneAction,
    TransformRelativePositionLens,
    Vec3,
    Vec3
);

relative_tween_fns!(
    scale,
    Transform,
    TweenDoneAction,
    TransformRelativeScaleLens,
    Vec3,
    Vec3
);

relative_tween_fns!(
    text_color,
    Text,
    TweenDoneAction,
    TextRelativeColorLens,
    Vec<Color>,
    Color
);

relative_tween_fns!(
    spritesheet_color,
    TextureAtlasSprite,
    TweenDoneAction,
    SpriteSheetRelativeColorLens,
    Color,
    Color
);

relative_tween_fns!(
    sprite_color,
    Sprite,
    TweenDoneAction,
    SpriteRelativeColorLens,
    Color,
    Color
);

relative_tween_fns!(
    ui_bg_color,
    BackgroundColor,
    TweenDoneAction,
    UiBackgroundColorLens,
    Color,
    Color
);
