use bevy::{
    math::Vec3Swizzles,
    prelude::*,
};

#[derive(Copy, Clone, Debug)]
pub struct InputMap {
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_sprint: KeyCode,
    pub key_jump: KeyCode,
    pub key_fly: KeyCode,
    pub key_crouch: KeyCode,
}


impl Default for InputMap {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_sprint: KeyCode::LShift,
            key_crouch: KeyCode::LControl,
            key_jump: KeyCode::Space,
            key_fly: KeyCode::F,
            key_up: KeyCode::Space,
            key_down: KeyCode::LControl,
            sensitivity: 0.001,
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerController {
    pub grounded: bool,
    pub fly: bool,
    pub jump: bool,
    pub sprint: bool,
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
    pub movement: Vec3,
    pub ground_tick: u8,
    pub stop_speed: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            grounded: false,
            fly: false,
            jump: false,
            sprint: false,
            fly_speed: 10.0,
            fast_fly_speed: 30.0,
            gravity: 23.0,
            walk_speed: 10.0,
            run_speed: 30.0,
            fwd_speed: 60.0,
            side_speed: 60.0,
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
            movement: Vec3::ZERO,
            ground_tick: 0,
            stop_speed: 1.0,
            jump_speed: 18.5,
        }
    }
}
