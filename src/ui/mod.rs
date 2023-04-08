use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

use crate::{state::PersistReset, AppSize};

mod game_over;
mod menu;

pub fn ui_plugin(app: &mut App) {
    app.fn_plugin(menu::menu_plugin)
        .fn_plugin(game_over::game_over_ui_plugin)
        .add_startup_system(setup_root_node)
        .add_system(resize_root_node.run_if(
            resource_exists::<RootUiNode>().and_then(resource_exists_and_changed::<AppSize>()),
        ));
}

#[derive(Resource)]
pub struct RootUiNode(pub Entity);

fn setup_root_node(mut cmd: Commands, size: Res<AppSize>) {
    let root = cmd
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(size.x), Val::Px(size.y)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(PersistReset)
        .id();

    cmd.insert_resource(RootUiNode(root));
}

fn resize_root_node(size: Res<AppSize>, root: Res<RootUiNode>, mut style_q: Query<&mut Style>) {
    let mut style = style_q
        .get_mut(root.0)
        .expect("Root node should always exist");
    style.size = Size::new(Val::Px(size.x), Val::Px(size.y));
}
