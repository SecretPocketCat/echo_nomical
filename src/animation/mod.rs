use bevy::prelude::*;
use bevy_tweening::{component_animator_system, TweeningPlugin};

mod tween;
mod tween_events;

pub use tween::*;
pub use tween_events::TweenDoneAction;

pub fn animation_plugin(app: &mut App) {
    app.add_plugin(TweeningPlugin)
        .add_system(tween_events::on_tween_completed)
        .add_system(component_animator_system::<TextureAtlasSprite>);
}
