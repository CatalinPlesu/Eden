use std::f32::consts::FRAC_PI_2;

use crate::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

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
    input_map: Res<InputMap>,
    mut windows: ResMut<Windows>,
    mut mouse_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerController>,
) {
    for mut player_input in query.iter_mut() {
        let window = windows.get_primary_mut().unwrap();
        if window.is_focused() {
            let mut mouse_delta = Vec2::ZERO;
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
            mouse_delta *= input_map.sensitivity;

            player_input.pitch = (player_input.pitch - mouse_delta.y)
                .clamp(-FRAC_PI_2 + 0.001953125, FRAC_PI_2 - 0.001953125);
            player_input.yaw = player_input.yaw - mouse_delta.x;
        }

        player_input.movement = Vec3::ZERO;

        if keyboard_input.pressed(input_map.key_forward) {
            player_input.movement.z += 1.;
        }
        if keyboard_input.pressed(input_map.key_back) {
            player_input.movement.z -= 1.;
        }
        if keyboard_input.pressed(input_map.key_right) {
            player_input.movement.x += 1.;
        }
        if keyboard_input.pressed(input_map.key_left) {
            player_input.movement.x -= 1.;
        }

        if player_input.fly {
            if keyboard_input.pressed(input_map.key_up) {
                player_input.movement.y += 1.;
            }
            if keyboard_input.pressed(input_map.key_down) {
                player_input.movement.y -= 1.;
            }
        } else {
            if key_input.pressed(input_map.key_jump) {
                player_input.jump = true;
                // player_input.movement.y = 1.;
            }
            // else {
            //     player_input.movement.y = -1.;
            // }
        }

        if player_input.movement.length_squared() != 0.0 {
            player_input.movement = player_input.movement.normalize();
        }

        if key_input.pressed(input_map.key_sprint) {
            player_input.sprint = true;
        } else {
            player_input.sprint = false;
        }
        if key_input.just_pressed(input_map.key_fly) {
            player_input.fly = !player_input.fly;
        }
    }
}
