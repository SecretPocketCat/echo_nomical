use bevy::prelude::*;

use crate::{render::camera::PrimaryCamera, state::AppState, AppSize};

use self::{
    level::{setup_level, update_score, ReachedLevel},
    map::TileType,
};

pub mod level;
pub mod map;

pub use map::Map;
pub const TILE_SIZE: f32 = 40.;
pub const HALF_TILE_SIZE: f32 = 20.;

pub fn lvl_plugin(app: &mut App) {
    app.init_resource::<ReachedLevel>()
        .insert_resource(Map::default())
        .add_systems(
            (update_map, setup_level, sync_camera)
                .chain()
                .in_schedule(OnEnter(AppState::Game)),
        )
        .add_system(
            sync_camera
                .run_if(resource_changed::<AppSize>())
                .in_set(OnUpdate(AppState::Game)),
        )
        .add_system(update_score);
}

pub fn update_map(mut map_resource: ResMut<Map>, reached: Res<ReachedLevel>) {
    bevy::log::info!("Creating map");
    let map_scale = 16. + 6. * reached.0 as f32;
    let map_size = (Vec2::new(16.0f32, 12.0f32).normalize() * map_scale).as_ivec2();
    loop {
        if let Some(new_map) = map::generate(map_size.x, map_size.y, reached.0) {
            let wall_count = new_map
                .tiles
                .iter()
                .filter(|&x| x == &TileType::Wall)
                .count();
            if wall_count as f32 / (new_map.tiles.len() as f32) < 0.6 {
                *map_resource = new_map;
                break;
            }
        }
    }
}

pub fn sync_camera(
    map: Res<Map>,
    bounds: Res<AppSize>,
    mut camera_transform: Query<&mut Transform, With<PrimaryCamera>>,
) {
    let physical_map_size = IVec2::new(map.width, map.height).as_vec2() * TILE_SIZE;
    camera_transform.single_mut().translation =
        (Vec2::new(map.width as f32, map.height as f32) * HALF_TILE_SIZE).extend(999.9);
    let scale_factor = (physical_map_size / bounds.0).max_element();
    camera_transform.single_mut().scale = Vec2::splat(scale_factor).extend(1.0);
}
