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
                    controller.velocity = controller.movement.normalize() * max_speed;
                    controller.velocity.y = controller.jump_speed;
                    controller.jump = false;
                } 
                else {
                    controller.movement.y = -0.85;
                    controller.velocity = controller.movement.normalize() * max_speed;
                    controller.velocity.y = -controller.gravity * dt;
                }
            }

            let vel = controller.velocity.x * right
                + controller.velocity.y * Vec3::Y
                + controller.velocity.z * fwd;
            rb_velocity.linvel = vel.into();
        }
    }
}
