use std::time::{Duration, Instant};

use bevy::{
    prelude::{
        default, AnimationPlayer, Children, Commands, DespawnRecursiveExt, Entity, EventWriter,
        Query, Res, ResMut, Transform, Vec2, Vec3, With, Without,
    },
    scene::SceneBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use crate::{
    events::AudioEvent,
    game::{
        audio::AudioClipAssets, models::ModelAssets, player::PLAYER_SIZE,
        utils::find_animation_player,
    },
};

use super::{
    components::{Enemy, EnemyAnimator},
    resources::EnemySpawnTimer,
    ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES,
};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    audio_clips: Res<AudioClipAssets>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let available_width = window.width() - PLAYER_SIZE;
        let available_height = window.height() - PLAYER_SIZE;
        let random_x_full = fastrand::f32() * available_width;
        let random_y_full = fastrand::f32() * available_height;

        let random_x = if random_x_full > available_width / 2.0 {
            random_x_full + PLAYER_SIZE
        } else {
            random_x_full
        };

        let random_y = if random_y_full > available_height / 2.0 {
            random_y_full + PLAYER_SIZE
        } else {
            random_y_full
        };

        commands.spawn((
            SceneBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                scene: model_assets.enemy.clone_weak(),
                ..default()
            },
            Enemy {
                direction: Vec2::new(fastrand::f32(), fastrand::f32()).normalize(),
                bounce_audio_clip_1: audio_clips.enemy_bounds_1.clone_weak(),
                bounce_audio_clip_2: audio_clips.enemy_bounds_2.clone_weak(),
                idle_animation_clip: model_assets.enemy_animation.clone_weak(),
                spawn_time: Instant::now(),
                delay_animation_start: Duration::from_millis(fastrand::u64(0..2000_u64)),
            },
        ));
    }
}

pub fn init_enemy_animation(
    enemy_query: Query<(Entity, &Enemy), (With<Enemy>, Without<EnemyAnimator>)>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    for (enemy_entity, enemy) in enemy_query.iter() {
        if enemy.spawn_time.elapsed() < enemy.delay_animation_start {
            continue;
        }
        if let Some(animation_player_entity) =
            find_animation_player(enemy_entity, &children_query, &animation_player_query)
        {
            if let Ok(mut animation_player) =
                animation_player_query.get_mut(animation_player_entity)
            {
                commands.entity(enemy_entity).insert(EnemyAnimator {
                    animation_player_entity,
                });
                animation_player
                    .play(enemy.idle_animation_clip.clone_weak())
                    .repeat();
            }
        }
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn_recursive();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SFX
        if direction_changed {
            // Randomly play one of the two sound effects.
            let clip = if fastrand::f32() > 0.5 {
                enemy.bounce_audio_clip_1.clone_weak()
            } else {
                enemy.bounce_audio_clip_2.clone_weak()
            };
            audio_event.send(AudioEvent { clip })
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let left = 0.0 + half_enemy_size;
    let right = window.width() - half_enemy_size;
    let top = 0.0 + half_enemy_size;
    let bottom = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // Bound the enemy x position
        if translation.x < left {
            translation.x = left;
        } else if translation.x > right {
            translation.x = right;
        }
        // Bound the enemy y position
        if translation.y < top {
            translation.y = top;
        } else if translation.y > bottom {
            translation.y = bottom;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    audio_clips: Res<AudioClipAssets>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = fastrand::f32() * window.width();
        let random_y = fastrand::f32() * window.height();

        commands.spawn((
            SceneBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                scene: model_assets.enemy.clone_weak(),
                ..default()
            },
            Enemy {
                direction: Vec2::new(fastrand::f32(), fastrand::f32()).normalize(),
                bounce_audio_clip_1: audio_clips.enemy_bounds_1.clone_weak(),
                bounce_audio_clip_2: audio_clips.enemy_bounds_2.clone_weak(),
                idle_animation_clip: model_assets.player_animation.clone_weak(),
                spawn_time: Instant::now(),
                delay_animation_start: Duration::default(),
            },
        ));
    }
}
