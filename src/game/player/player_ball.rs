use bevy::{
    prelude::{default, Bundle, Commands, Res, Transform, Vec2, Vec3},
    scene::SceneBundle,
};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

use super::components::Player;

#[derive(Bundle)]
pub struct PlayerActorBundle {
    pub player: Player,
    pub scene_bundle: SceneBundle,
}

pub struct PlayerBallDefault;

impl BundledActor<PlayerActorBundle> for PlayerBallDefault {
    fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> PlayerActorBundle {
        return PlayerActorBundle {
            scene_bundle: SceneBundle {
                scene: model_assets.player.clone_weak(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0),
                ..default()
            },
            player: Player {
                explosion_audio_clip: audio_clips.explosion.clone_weak(),
                idle_animation_clip: model_assets.player_animation.clone_weak(),
                input_direction: Vec3::ZERO,
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
