use bevy::prelude::{
    Input, KeyCode, MouseButton, Query, Res, ScanCode, Touches, Transform, Vec2, Vec3, With,
};
use bevy::window::{PrimaryWindow, Window};
use bevy_rapier3d::prelude::{ExternalImpulse, Velocity};

use crate::game::player::components::Player;
use crate::game::player::{PLAYER_FORCE, PLAYER_SIZE, PLAYER_SPEED};

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
    mut player_query: Query<(&Transform, &mut ExternalImpulse, &mut Velocity), With<Player>>,
) {
    if let Ok((player_transform, mut player_impulse, mut player_velocity)) =
        player_query.get_single_mut()
    {
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

        player_impulse.impulse = direction * PLAYER_FORCE * player_transform.scale.x;
        // player_velocity.linvel += direction * PLAYER_FORCE * player_transform.scale.x;
        player_velocity.linvel = player_velocity
            .linvel
            .clamp_length_max(PLAYER_SPEED * player_transform.scale.x);
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut translation = player_transform.translation;

        let size = PLAYER_SIZE * player_transform.scale.x;

        let left = -size;
        let right = window.width() + size;
        let top = -size;
        let bottom = window.height() + size;

        // Bound the player x position
        if translation.x < left {
            translation.x = left + size * 2.0;
        } else if translation.x > right {
            translation.x = right - size * 2.0;
        }
        // Bound the players y position.
        if translation.y < top {
            translation.y = top + size * 2.0;
        } else if translation.y > bottom {
            translation.y = bottom - size * 2.0;
        }

        player_transform.translation = translation;
    }
}
