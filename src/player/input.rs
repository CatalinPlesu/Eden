use std::f32::consts::FRAC_PI_2;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    input::mouse::MouseMotion,
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use flagset::{flags, FlagSet};

flags! {
    pub enum PlayerInputFlags: u32 {
        Jump,
        Sprint,
        Fly,
    }
}

#[derive(Component, Default)]
pub struct PlayerInput {
    pub movement: Vec3,
    pub flags: FlagSet<PlayerInputFlags>,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, TypeUuid)]
#[uuid = "9ac18a62-063a-4fa1-9575-d295ce69997b"]
pub struct Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::Q,
            key_down: KeyCode::E,
            key_sprint: KeyCode::LShift,
            key_jump: KeyCode::Space,
            key_fly: KeyCode::F,
            key_crouch: KeyCode::LControl,
            sensitivity: 0.001,
        }
    }
}

fn get_pressed(key_input: &Res<Input<KeyCode>>, key: KeyCode) -> f32 {
    if key_input.pressed(key) {
        1.0
    } else {
        0.0
    }
}

fn get_axis(key_input: &Res<Input<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}

pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

pub fn player_input_system(
    key_input: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut windows: ResMut<Windows>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<&mut PlayerInput>)
{
    for mut player_input in query.iter_mut() {
        let window = windows.get_primary_mut().unwrap();
        if window.is_focused() {
            let mut mouse_delta = Vec2::ZERO;
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
            mouse_delta *= config.sensitivity;

            player_input.pitch = (player_input.pitch - mouse_delta.y).clamp(
                -FRAC_PI_2 + 0.001953125,
                FRAC_PI_2 - 0.001953125,
                );
            player_input.yaw = player_input.yaw - mouse_delta.x;
        }

        player_input.movement = Vec3::new(
            get_axis(&key_input, config.key_right, config.key_left),
            get_axis(&key_input, config.key_up, config.key_down),
            get_axis(&key_input, config.key_forward, config.key_back),
            );
        player_input.flags.clear();
        if key_input.pressed(config.key_sprint) {
            player_input.flags |= PlayerInputFlags::Sprint;
        }
        if key_input.pressed(config.key_jump) {
            player_input.flags |= PlayerInputFlags::Jump;
        }
        if key_input.just_pressed(config.key_fly) {
            player_input.flags |= PlayerInputFlags::Fly;
        }
    }
}
