use bevy::prelude::*;
use bevy_tweening::*;

use super::tween_macros::*;

relative_lens!(Transform, Vec3, TransformRelativeScaleLens, scale);
relative_lens!(Transform, Vec3, TransformRelativePositionLens, translation);

#[derive(Default)]
pub struct TransformRelativeByPositionLens {
    start: Vec3,
    end: Vec3,
    move_by: Vec3,
}

impl TransformRelativeByPositionLens {
    #[allow(dead_code)]
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
    pub start: Option<Vec<Color>>,
    pub end: Color,
}

impl TextRelativeColorLens {
    #[allow(dead_code)]
    pub fn relative(end: Color) -> Self {
        Self { start: None, end }
    }
}

impl Lens<Text> for TextRelativeColorLens {
    fn lerp(&mut self, target: &mut Text, ratio: f32) {
        for i in 0..target.sections.len() {
            if let Some(col) = self.start.as_ref().unwrap().get(i) {
                target.sections[i].style.color = lerp_color(*col, self.end, ratio);
            }
        }
    }

    fn update_on_tween_start(&mut self, target: &Text) {
        self.start
            .get_or_insert_with(|| target.sections.iter().map(|s| s.style.color).collect());
    }
}

color_lens!(Sprite, SpriteRelativeColorLens, color);
color_lens!(TextureAtlasSprite, SpriteSheetRelativeColorLens, color);
color_lens!(BackgroundColor, UiBackgroundColorLens, 0);

fn lerp_color(from: Color, to: Color, ratio: f32) -> Color {
    let start: Vec4 = from.into();
    let end: Vec4 = to.into();
    start.lerp(end, ratio).into()
}
