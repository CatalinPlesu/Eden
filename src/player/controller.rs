use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::*;

fn look_quat(pitch: f32, yaw: f32) -> Quat {
    return Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
}

pub fn sync_player_camera_system(
    controller_query: Query<(&PlayerController, &RigidBodyPositionComponent)>,
    mut camera_query: Query<&mut Transform, With<PerspectiveProjection>>,
) {
    for (controller, rb_position) in controller_query.iter() {
        for mut transform in camera_query.iter_mut() {
            transform.translation =
                Vec3::from(rb_position.position.translation) + Vec3::new(0.0, 2.0, 0.0);
            transform.rotation = look_quat(controller.pitch, controller.yaw);
        }
    }
}

fn friction(lateral_speed: f32, friction: f32, stop_speed: f32, dt: f32, velocity: &mut Vec3) {
    let control = f32::max(lateral_speed, stop_speed);
    let drop = control * friction * dt;
    let new_speed = f32::max((lateral_speed - drop) / lateral_speed, 0.0);
    velocity.x *= new_speed;
    velocity.z *= new_speed;
}

fn accelerate(wish_dir: Vec3, wish_speed: f32, accel: f32, dt: f32, velocity: &mut Vec3) {
    let vel_proj = Vec3::dot(*velocity, wish_dir);
    let add_speed = wish_speed - vel_proj;
    if add_speed <= 0.0 {
        return;
    }

    let accel_speed = f32::min(accel * wish_speed * dt, add_speed);
    let wish_dir = wish_dir * accel_speed;
    velocity.x += wish_dir.x;
    velocity.z += wish_dir.z;
}

pub fn player_move_system(
    time: Res<Time>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut query: Query<(
        Entity,
        &mut PlayerController,
        &ColliderShapeComponent,
        &RigidBodyPositionComponent,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    let dt = time.delta_seconds();

    for (entity, mut controller, collider, rb_position, mut rb_velocity) in query.iter_mut() {
        let rot = look_quat(controller.pitch, controller.yaw);
        let right = rot * Vec3::X;
        let fwd = rot * -Vec3::Z;
        let pos: Vec3 = rb_position.position.translation.into();

        if controller.fly {
            if controller.movement == Vec3::ZERO {
                let friction = controller.fly_friction.clamp(0.0, 1.0);
                controller.velocity *= 1.0 - friction;
                if controller.velocity.length_squared() < 1e-6 {
                    controller.velocity = Vec3::ZERO;
                }
            } else {
                let fly_speed = if controller.fly {
                    controller.fast_fly_speed
                } else {
                    controller.fly_speed
                };
                controller.velocity = controller.movement.normalize() * fly_speed;
            }
            let vel = controller.velocity.x * right
                + controller.velocity.y * Vec3::Y
                + controller.velocity.z * fwd;
            rb_velocity.linvel = vel.into();
        } else {
            if controller.movement == Vec3::ZERO {
                let friction = controller.friction.clamp(0.0, 1.0);
                controller.velocity *= 1.0 - friction;
                if controller.velocity.length_squared() < 1e-6 {
                    controller.velocity = Vec3::new(0., -controller.gravity, 0.);
                }
            } else {
                let max_speed = if controller.sprint {
                    controller.run_speed
                } else {
                    controller.walk_speed
                };

                if controller.jump {
                    controller.velocity.y = controller.jump_speed;
                    controller.jump = false;
                } else {
                    controller.movement.y = -0.85;
                    controller.velocity = controller.movement.normalize() * max_speed;
                    controller.velocity.y = -controller.gravity * dt;
                }
            }

            let vel = controller.velocity.x * right
                + controller.velocity.y * Vec3::Y
                + controller.velocity.z * fwd;
            rb_velocity.linvel = vel.into();

            // if let Some(capsule) = collider.as_capsule() {
            //     let mut init_vel = controller.velocity;
            //     let mut end_vel = init_vel;
            //     let lateral_speed = init_vel.xz().length();

            //     // Capsule cast downwards to find ground
            //     let mut ground_hit = None;
            //     let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
            //     let cast_capsule =
            //         Capsule::new(capsule.segment.a, capsule.segment.b, capsule.radius * 1.1);
            //     let cast_pos = (pos, rot).into();
            //     let cast_dir = Vec3::new(0.0, -1.0, 0.0).into();
            //     let max_dist = 0.125;
            //     let groups = InteractionGroups::all().with_memberships(u32::MAX ^ RAPIER_TANGERINE_GROUP);

            //     if let Some((handle, hit)) = query_pipeline.cast_shape(
            //         &collider_set,
            //         &cast_pos,
            //         &cast_dir,
            //         &cast_capsule,
            //         max_dist,
            //         groups,
            //         // Filter to prevent self-collisions
            //         Some(&|hit_collider| hit_collider.entity() != entity),
            //     ) {
            //         ground_hit = Some(hit);
            //         println!("hit {:?}", ground_hit);
            //     }
            // }
        }
    }
}
