use std::time::{Duration, Instant};

use bevy::{
    prelude::{default, Bundle, Commands, Res, Transform, Vec2},
    scene::SceneBundle,
};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

use super::components::Enemy;

#[derive(Bundle)]
pub struct EnemyActorBundle {
    pub enemy: Enemy,
    pub scene_bundle: SceneBundle,
}

pub struct EnemyBallDefault;

impl BundledActor<EnemyActorBundle> for EnemyBallDefault {
    fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> EnemyActorBundle {
        return EnemyActorBundle {
            scene_bundle: SceneBundle {
                scene: model_assets.enemy.clone_weak(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0),
                ..default()
            },
            enemy: Enemy {
                direction: Vec2::new(fastrand::f32(), fastrand::f32()).normalize(),
                bounce_audio_clip_1: audio_clips.enemy_bounds_1.clone_weak(),
                bounce_audio_clip_2: audio_clips.enemy_bounds_2.clone_weak(),
                idle_animation_clip: model_assets.enemy_animation.clone_weak(),
                spawn_time: Instant::now(),
                delay_animation_start: Duration::from_millis(fastrand::u64(0..2000_u64)),
            },
        };
    }
    fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}
