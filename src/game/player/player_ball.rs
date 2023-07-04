use bevy::{
    prelude::{default, Bundle, Commands, Res, Transform, Vec2, Vec3},
    scene::SceneBundle,
};
use bevy_rapier2d::prelude::{
    Collider, CollisionGroups, Damping, ExternalImpulse, Group, RigidBody, Velocity,
};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

use super::{
    components::{Player, PlayerHealth},
    PLAYER_SIZE,
};

#[derive(Bundle)]
pub struct PlayerActorBundle {
    pub player: Player,
    pub scene_bundle: SceneBundle,
    pub rigid_body: RigidBody,
    pub external_impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub damping: Damping,
    pub health: PlayerHealth,
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
                shrink_audio_clip: audio_clips.shrink.clone_weak(),
                idle_animation_clip: model_assets.player_animation.clone_weak(),
                input_direction: Vec3::ZERO,
            },
            rigid_body: RigidBody::Dynamic,
            external_impulse: ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            },
            velocity: Velocity::default(),
            collider: Collider::ball(PLAYER_SIZE / 2.0),
            collision_group: CollisionGroups::new(Group::GROUP_2, Group::ALL),
            damping: Damping {
                linear_damping: 0.8,
                angular_damping: 0.3,
            },
            health: PlayerHealth {
                value: PLAYER_SIZE / 2.0,
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
