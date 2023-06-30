use bevy::prelude::{
    AnimationPlayer, Children, Commands, DespawnRecursiveExt, Entity, EventWriter, Input, KeyCode,
    Query, Res, ResMut, ScanCode, Transform, Vec2, Vec3, With, Without,
};
use bevy::time::Time;
use bevy::window::{PrimaryWindow, Window};

use super::components::{Player, PlayerAnimator};
use super::player_ball::PlayerBallDefault;
use crate::events::{AudioEvent, GameOverEvent};
use crate::game::actor::BundledActor;
use crate::game::audio::AudioClipAssets;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::models::ModelAssets;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;
use crate::game::utils::find_animation_player;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
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

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    audio_clips: Res<AudioClipAssets>,
) {
    let window = window_query.get_single().unwrap();
    let spawn_position = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    PlayerBallDefault::spawn_bundle(&mut commands, &audio_clips, &model_assets, spawn_position);
}

pub fn init_player_animation(
    player_query: Query<(Entity, &Player), (With<Player>, Without<PlayerAnimator>)>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    for (player_entity, player) in player_query.iter() {
        if let Some(animation_player_entity) =
            find_animation_player(player_entity, &children_query, &animation_player_query)
        {
            if let Ok(mut animation_player) =
                animation_player_query.get_mut(animation_player_entity)
            {
                commands.entity(player_entity).insert(PlayerAnimator {
                    animation_player_entity,
                });
                animation_player
                    .play(player.idle_animation_clip.clone_weak())
                    .repeat();
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}

pub fn player_movement(
    keyboard_input_scan_code: Res<Input<ScanCode>>,
    keyboard_input_key_code: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

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

        // if direction.length() > 0.0 {
        //     direction = direction.normalize();
        // }
        direction = direction.normalize_or_zero();

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    if let Ok((player_entity, player_transform, player)) = player_query.get_single_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                audio_event.send(AudioEvent {
                    clip: player.explosion_audio_clip.clone_weak(),
                });
                commands.entity(player_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
                game_over_event_writer.send(GameOverEvent {});
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform, &Star), With<Star>>,
    mut score: ResMut<Score>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform, star) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                score.value += 1;
                audio_event.send(AudioEvent {
                    clip: star.collect_audio_clip.clone_weak(),
                });
                commands.entity(star_entity).despawn_recursive();
            }
        }
    }
}