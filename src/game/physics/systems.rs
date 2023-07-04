use bevy::prelude::{
    warn, Commands, DespawnRecursiveExt, Entity, EventReader, Query, Res, Transform, Vec2, Vec3,
    With,
};
use bevy::window::{PrimaryWindow, Window, WindowResized};

use crate::game::audio::AudioClipAssets;
use crate::game::models::ModelAssets;

use super::components::{
    BottomWallDefault, LeftWallDefault, RightWallDefault, TopWallDefault, WallType,
};

pub fn spawn_level_walls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    audio_clips: Res<AudioClipAssets>,
) {
    let window = window_query.get_single().unwrap();

    LeftWallDefault::spawn_bundle(
        &mut commands,
        &audio_clips,
        &model_assets,
        Vec2::new(0.0, 0.0),
    );
    RightWallDefault::spawn_bundle(
        &mut commands,
        &audio_clips,
        &model_assets,
        Vec2::new(window.width(), 0.0),
    );
    TopWallDefault::spawn_bundle(
        &mut commands,
        &audio_clips,
        &model_assets,
        Vec2::new(0.0, 0.0),
    );
    BottomWallDefault::spawn_bundle(
        &mut commands,
        &audio_clips,
        &model_assets,
        Vec2::new(0.0, window.height()),
    );
}

pub fn despawn_level_walls(mut commands: Commands, wall_query: Query<Entity, With<WallType>>) {
    if let Ok(wall_entity) = wall_query.get_single() {
        commands.entity(wall_entity).despawn_recursive();
    }
}

pub fn update_walls_position(
    mut resize_reader: EventReader<WindowResized>,
    mut wall_query: Query<(&mut Transform, &WallType), With<WallType>>,
) {
    for e in resize_reader.iter() {
        for (mut wall_transform, wall_type) in wall_query.iter_mut() {
            match wall_type {
                WallType::Left => {
                    warn!("Left wall changed {}", wall_transform.translation);
                    wall_transform.translation = Vec3::ZERO;
                }
                WallType::Right => {
                    warn!("Right wall changed {}", wall_transform.translation);
                    wall_transform.translation = Vec3::new(e.width, 0.0, 0.0);
                }
                WallType::Top => {
                    warn!("Top wall changed {}", wall_transform.translation);
                    wall_transform.translation = Vec3::ZERO;
                }
                WallType::Bottom => {
                    warn!("Bottom wall changed {}", wall_transform.translation);
                    wall_transform.translation = Vec3::new(0.0, e.height, 0.0);
                }
            }
        }
    }
}
