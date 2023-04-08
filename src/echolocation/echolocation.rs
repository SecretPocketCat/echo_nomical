use bevy::{prelude::*, utils::HashSet};

use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    animation::{get_relative_sprite_color_anim, TweenDoneAction},
    input::actions::PlayerAction,
    time::time::{ScaledTime, ScaledTimeDelta, ScaledTimeFields},
};

#[derive(Component)]
pub(super) struct Echolocate {
    elapsed_time: f32,
    max_time: f32,
    max_radius: f32,
    used_rays: HashSet<u16>,
    ray_count: u16,
}

impl Echolocate {
    fn radius(&self) -> f32 {
        self.max_radius * (self.elapsed_time / self.max_time)
    }
}

pub(super) fn echolocate(
    mut cmd: Commands,
    input_q: Query<(&ActionState<PlayerAction>, &GlobalTransform)>,
) {
    for (_, t) in input_q
        .iter()
        .filter(|(input, ..)| input.just_released(PlayerAction::Echo))
    {
        cmd.spawn((
            Echolocate {
                elapsed_time: 0.,
                max_time: 2.,
                max_radius: 300.,
                used_rays: default(),
                ray_count: 500,
            },
            TransformBundle::from_transform(Transform::from_translation(t.translation())),
        ));
    }
}

pub(super) fn test_intersections(
    mut cmd: Commands,
    rapier_context: Res<RapierContext>,
    mut echo_q: Query<(Entity, &mut Echolocate, &GlobalTransform)>,
    // coll_q: Query<(&Collider)>,
    time: ScaledTime,
) {
    for (e, mut echo, t) in echo_q.iter_mut() {
        let pos = t.translation().truncate();

        let radius = echo.radius();
        let shape = Collider::ball(radius);

        rapier_context.intersections_with_shape(
            pos,
            0.,
            &shape,
            QueryFilter::exclude_kinematic(),
            |_| {
                let ray_step = 360. / echo.ray_count as f32;
                for i in 0..echo.ray_count {
                    if echo.used_rays.contains(&i) {
                        continue;
                    }

                    let dir = Vec2::from_angle(i as f32 * ray_step);
                    if let Some(hit) = rapier_context.cast_ray_and_get_normal(
                        pos,
                        dir,
                        radius,
                        false,
                        // todo: ignore player as enemies will be kinematic too
                        QueryFilter::exclude_kinematic(),
                    ) {
                        echo.used_rays.insert(i);
                        cmd.spawn(SpriteBundle {
                            transform: Transform::from_translation(
                                hit.1
                                    .point
                                    .extend(20. + time.time().elapsed_seconds() / 100.),
                            )
                            .with_rotation(Quat::from_rotation_z(
                                Vec2::Y.angle_between(hit.1.normal),
                            )),
                            sprite: Sprite {
                                color: Color::GREEN,
                                custom_size: Some(Vec2::new(9., 3.)),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(get_relative_sprite_color_anim(
                            Color::ANTIQUE_WHITE,
                            3500,
                            TweenDoneAction::DespawnRecursive,
                        ));
                    }
                }

                false
            },
        );

        echo.elapsed_time += time.scaled_delta_seconds();

        if echo.elapsed_time >= echo.max_time || echo.used_rays.len() >= echo.ray_count as usize {
            cmd.entity(e).despawn_recursive();
        }
    }
}
