use bevy::prelude::{
    Input, KeyCode, MouseButton, Query, Res, ScanCode, Touches, Transform, Vec2, Vec3, With,
};
use bevy::time::Time;
use bevy::window::{PrimaryWindow, Window};

use crate::game::player::components::Player;
use crate::game::player::{PLAYER_SIZE, PLAYER_SPEED};

pub enum EScanCode {
    W = 17,
    A = 30,
    S = 31,
    D = 32,
    Left = 105,
    Right = 106,
    Up = 103,
    Down = 108,
}

pub fn player_movement(
    keyboard_input_scan_code: Res<Input<ScanCode>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    touches: Res<Touches>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if let Ok(window) = window_query.get_single() {
            if mouse_button_input.pressed(MouseButton::Left) {
                if let Some(cursor_position) = window.cursor_position() {
                    let diff_x = cursor_position.x - player_transform.translation.x;
                    let diff_y = cursor_position.y - player_transform.translation.y;
                    let normilized_diff = Vec3::new(diff_x, diff_y, 0.0).normalize_or_zero();
                    direction += normilized_diff;
                }
            }
            if let Some(touch_position) = touches.first_pressed_position() {
                let touch_position =
                    Vec2::new(touch_position.x, window.height() - touch_position.y); // tranform y coordinate to be the same as mouse coordinates
                let diff_x = touch_position.x - player_transform.translation.x;
                let diff_y = touch_position.y - player_transform.translation.y;
                let normilized_diff = Vec3::new(diff_x, diff_y, 0.0).normalize_or_zero();
                direction += normilized_diff;
            }
        }

        if keyboard_input_scan_code.any_pressed([
            ScanCode(EScanCode::A as u32),
            ScanCode(EScanCode::Left as u32),
        ]) || keyboard_input_key_code.any_pressed([KeyCode::A, KeyCode::Left])
        {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        if keyboard_input_scan_code.any_pressed([
            ScanCode(EScanCode::D as u32),
            ScanCode(EScanCode::Right as u32),
        ]) || keyboard_input_key_code.any_pressed([KeyCode::D, KeyCode::Right])
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input_scan_code.any_pressed([
            ScanCode(EScanCode::W as u32),
            ScanCode(EScanCode::Up as u32),
        ]) || keyboard_input_key_code.any_pressed([KeyCode::W, KeyCode::Up])
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input_scan_code.any_pressed([
            ScanCode(EScanCode::S as u32),
            ScanCode(EScanCode::Down as u32),
        ]) || keyboard_input_key_code.any_pressed([KeyCode::S, KeyCode::Down])
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        direction = direction.normalize_or_zero();

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let left = 0.0 + half_player_size;
        let right = window.width() - half_player_size;
        let top = 0.0 + half_player_size;
        let bottom = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < left {
            translation.x = left;
        } else if translation.x > right {
            translation.x = right;
        }
        // Bound the players y position.
        if translation.y < top {
            translation.y = top;
        } else if translation.y > bottom {
            translation.y = bottom;
        }

        player_transform.translation = translation;
    }
}
