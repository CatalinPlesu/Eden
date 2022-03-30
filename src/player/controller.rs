use bevy::{
    math::Vec3Swizzles,
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::{PlayerInput, PlayerInputFlags};

pub enum MoveMode {
    Noclip,
    Ground,
}

#[derive(Component)]
pub struct PlayerController {
    pub move_mode: MoveMode,
    pub gravity: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub fwd_speed: f32,
    pub side_speed: f32,
    pub air_speed_cap: f32,
    pub air_accel: f32,
    pub max_air_speed: f32,
    pub accel: f32,
    pub friction: f32,
    pub friction_cutoff: f32,
    pub jump_speed: f32,
    pub fly_speed: f32,
    pub fast_fly_speed: f32,
    pub fly_friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub ground_tick: u8,
    pub stop_speed: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            move_mode: MoveMode::Ground,
            fly_speed: 10.0,
            fast_fly_speed: 30.0,
            gravity: 23.0,
            walk_speed: 10.0,
            run_speed: 30.0,
            fwd_speed: 30.0,
            side_speed: 30.0,
            air_speed_cap: 2.0,
            air_accel: 20.0,
            max_air_speed: 8.0,
            accel: 10.0,
            friction: 10.0,
            friction_cutoff: 0.1,
            fly_friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            ground_tick: 0,
            stop_speed: 1.0,
            jump_speed: 8.5,
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
    if add_speed <= 0.0 { return; }

    let accel_speed = f32::min(accel * wish_speed * dt, add_speed);
    let wish_dir = wish_dir * accel_speed;
    velocity.x += wish_dir.x;
    velocity.z += wish_dir.z;
}

pub fn player_look_system(
    mut query: Query<(&mut PlayerController, &PlayerInput)>
) {
    for (mut controller, input) in query.iter_mut() {
        controller.pitch = input.pitch;
        controller.yaw = input.yaw;
    }
}

fn look_quat(pitch: f32, yaw: f32) -> Quat {
    return Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
}

pub fn sync_player_camera_system(
    controller_query: Query<(&PlayerController, &RigidBodyPositionComponent)>,
    mut camera_query: Query<&mut Transform, With<PerspectiveProjection>>,
) {
    for (controller, rb_position) in controller_query.iter() {
        for mut transform in camera_query.iter_mut() {
            transform.translation = Vec3::from(rb_position.position.translation) + Vec3::new(0.0, 2.0, 0.0);
            transform.rotation = look_quat(controller.pitch, controller.yaw);
        }
    }
}

pub fn player_move_system(
    time: Res<Time>,
    query_pipeline: Res<QueryPipeline>, collider_query: QueryPipelineColliderComponentsQuery,
    mut query: Query<(
        Entity, &PlayerInput, &mut PlayerController, &ColliderShapeComponent,
        &RigidBodyPositionComponent, &mut RigidBodyVelocityComponent,
    )>,
) {
    let dt = time.delta_seconds();

    for (entity, input, mut controller, collider, rb_position, mut rb_velocity) in query.iter_mut() {

        if input.flags.contains(PlayerInputFlags::Fly) {
            controller.move_mode = match controller.move_mode {
                MoveMode::Noclip => { MoveMode::Ground }
                MoveMode::Ground => { MoveMode::Noclip }
            }
        }

        let rot = look_quat(input.pitch, input.yaw);
        let right = rot * Vec3::X;
        let fwd = rot * -Vec3::Z;
        let pos: Vec3 = rb_position.position.translation.into();

        match controller.move_mode {
            MoveMode::Noclip => {
                if input.movement == Vec3::ZERO {
                    let friction = controller.fly_friction.clamp(0.0, 1.0);
                    controller.velocity *= 1.0 - friction;
                    if controller.velocity.length_squared() < 1e-6 {
                        controller.velocity = Vec3::ZERO;
                    }
                } else {
                    let fly_speed = if input.flags.contains(PlayerInputFlags::Sprint) {
                        controller.fast_fly_speed
                    } else {
                        controller.fly_speed
                    };
                    controller.velocity = input.movement.normalize() * fly_speed;
                }
                let vel = controller.velocity.x * right
                    + controller.velocity.y * Vec3::Y
                    + controller.velocity.z * fwd;
                rb_velocity.linvel = vel.into();
            }

            MoveMode::Ground => {
                if let Some(capsule) = collider.as_capsule() {
                    let mut init_vel = controller.velocity;
                    let mut end_vel = init_vel;
                    let lateral_speed = init_vel.xz().length();

                    // Capsule cast downwards to find ground
                    let mut ground_hit = None;
                    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
                    let cast_capsule = Capsule::new(capsule.segment.a, capsule.segment.b, capsule.radius * 1.0625);
                    let cast_pos = (pos, rot).into();
                    let cast_dir = Vec3::new(0.0, -1.0, 0.0).into();
                    let max_dist = 0.125;
                    let groups = InteractionGroups::all();

                    if let Some((handle, hit)) = query_pipeline.cast_shape(
                        &collider_set, &cast_pos, &cast_dir, &cast_capsule, max_dist, groups,
                        // Filter to prevent self-collisions
                        Some(&|hit_collider| hit_collider.entity() != entity),
                    ) {
                        ground_hit = Some(hit);
                    }

                    let mut wish_dir = input.movement.z * controller.fwd_speed * fwd
                        + input.movement.x * controller.side_speed * right;
                    let mut wish_speed = wish_dir.length();
                    if wish_speed > 1e-6 { // Avoid division by zero
                        wish_dir /= wish_speed; // Effectively normalize, avoid length computation twice
                    }

                    let max_speed = if input.flags.contains(PlayerInputFlags::Sprint) {
                        controller.run_speed
                    } else {
                        controller.walk_speed
                    };

                    wish_speed = f32::min(wish_speed, max_speed);

                    if let Some(ground_hit) = ground_hit {
                        // Only apply friction after at least one tick, allows b-hopping without losing speed
                        if controller.ground_tick >= 1 {
                            if lateral_speed > controller.friction_cutoff {
                                friction(lateral_speed, controller.friction, controller.stop_speed, dt, &mut end_vel);
                            } else {
                                end_vel.x = 0.0;
                                end_vel.z = 0.0;
                            }
                            end_vel.y = 0.0;
                        }
                        accelerate(wish_dir, wish_speed, controller.accel, dt, &mut end_vel);
                        if input.flags.contains(PlayerInputFlags::Jump) {
                            // Simulate one update ahead, since this is an instant velocity change
                            init_vel.y = controller.jump_speed;
                            end_vel.y = init_vel.y - controller.gravity * dt;
                        }
                        // Increment ground tick but cap at max value
                        controller.ground_tick = controller.ground_tick.saturating_add(1);
                    } else {
                        controller.ground_tick = 0;
                        wish_speed = f32::min(wish_speed, controller.air_speed_cap);
                        accelerate(wish_dir, wish_speed, controller.air_accel, dt, &mut end_vel);
                        end_vel.y -= controller.gravity * dt;
                        let air_speed = end_vel.xz().length();
                        if air_speed > controller.max_air_speed {
                            let ratio = controller.max_air_speed / air_speed;
                            end_vel.x *= ratio;
                            end_vel.z *= ratio;
                        }
                    }

                    let rb_vel = (init_vel + end_vel) * 0.5;
                    controller.velocity = end_vel;
                    rb_velocity.linvel = rb_vel.into();
                }
            }
        }
    }
}
