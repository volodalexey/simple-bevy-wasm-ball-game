use bevy::prelude::{
    default, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut, Transform, With,
};
use bevy::scene::SceneBundle;
use bevy::time::Time;
use bevy::window::{PrimaryWindow, Window};
use fastrand;

use crate::game::audio::AudioClipAssets;
use crate::game::models::ModelAssets;
use crate::game::player::PLAYER_SIZE;

use super::components::Star;
use super::resources::StarSpawnTimer;
use super::NUMBER_OF_STARS;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    audio_clips: Res<AudioClipAssets>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
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
                scene: model_assets.star.clone(),
                ..default()
            },
            Star {
                collect_audio_clip: audio_clips.star_collect.clone(),
            },
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn_recursive();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    star_spawn_timer: Res<StarSpawnTimer>,
    audio_clips: Res<AudioClipAssets>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = fastrand::f32() * window.width();
        let random_y = fastrand::f32() * window.height();

        commands.spawn((
            SceneBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                scene: model_assets.star.clone(),
                ..default()
            },
            Star {
                collect_audio_clip: audio_clips.star_collect.clone(),
            },
        ));
    }
}
