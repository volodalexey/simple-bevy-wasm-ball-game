use bevy::{
    prelude::{
        AnimationPlayer, Children, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter,
        Query, Res, ResMut, Vec2, With, Without,
    },
    time::Time,
    window::{PrimaryWindow, Window},
};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{
    events::AudioEvent,
    game::{
        actor::BundledActor, audio::AudioClipAssets, models::ModelAssets,
        physics::components::WallType, player::PLAYER_SIZE, utils::find_animation_player,
    },
};

use super::{
    components::{Enemy, EnemyAnimator},
    enemy_ball::EnemyBallDefault,
    resources::EnemySpawnTimer,
    NUMBER_OF_ENEMIES,
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
        let spawn_position = Vec2::new(random_x, random_y);

        EnemyBallDefault::spawn_bundle(&mut commands, &audio_clips, &model_assets, spawn_position);
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

pub fn enemy_collide(
    enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
    mut audio_event: EventWriter<AudioEvent>,
    wall_query: Query<&WallType, With<WallType>>,
    mut collision_reader: EventReader<CollisionEvent>,
) {
    for collision_event in collision_reader.iter() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = collision_event {
            // each pair must contains at least one enemy
            let some_enemy: Option<&Enemy> = if let Ok((_, enemy)) = enemy_query.get(*entity_a) {
                Some(enemy)
            } else if let Ok((_, enemy)) = enemy_query.get(*entity_b) {
                Some(enemy)
            } else {
                None
            };
            let some_wall: Option<&WallType> = if let Ok(wall) = wall_query.get(*entity_a) {
                Some(wall)
            } else if let Ok(wall) = wall_query.get(*entity_b) {
                Some(wall)
            } else {
                None
            };
            if let Some(enemy) = some_enemy {
                if let Some(_) = some_wall {
                    // Randomly play one of the two sound effects.
                    let clip = if fastrand::f32() > 0.5 {
                        enemy.bounce_audio_clip_1.clone_weak()
                    } else {
                        enemy.bounce_audio_clip_2.clone_weak()
                    };
                    audio_event.send(AudioEvent { clip });
                } else {
                    let clip = enemy.hit_audio_clip_1.clone_weak();
                    audio_event.send(AudioEvent { clip });
                }
            }
        }
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

        let spawn_position = Vec2::new(random_x, random_y);

        EnemyBallDefault::spawn_bundle(&mut commands, &audio_clips, &model_assets, spawn_position);
    }
}
