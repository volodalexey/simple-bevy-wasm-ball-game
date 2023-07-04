use std::time::Duration;

use bevy::{
    prelude::{default, Bundle, Commands, Res, Transform, Vec2},
    scene::SceneBundle,
    utils::Instant,
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Group, Restitution, RigidBody, Velocity,
};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

use super::{
    components::{Enemy, EnemyHealth},
    ENEMY_SIZE, ENEMY_SPEED,
};

#[derive(Bundle)]
pub struct EnemyActorBundle {
    pub enemy: Enemy,
    pub scene_bundle: SceneBundle,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub restitution: Restitution,
    pub collision_events: ActiveEvents,
    pub health: EnemyHealth,
}

pub struct EnemyBallDefault;

impl BundledActor<EnemyActorBundle> for EnemyBallDefault {
    fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> EnemyActorBundle {
        let velocity = Vec2::new(fastrand::f32(), fastrand::f32()).normalize() * ENEMY_SPEED;

        return EnemyActorBundle {
            scene_bundle: SceneBundle {
                scene: model_assets.enemy.clone_weak(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0),
                ..default()
            },
            enemy: Enemy {
                bounce_audio_clip_1: audio_clips.enemy_bounds_1.clone_weak(),
                bounce_audio_clip_2: audio_clips.enemy_bounds_2.clone_weak(),
                hit_audio_clip_1: audio_clips.hit_clip_1.clone_weak(),
                idle_animation_clip: model_assets.enemy_animation.clone_weak(),
                spawn_time: Instant::now(),
                delay_animation_start: Duration::from_millis(fastrand::u64(0..2000_u64)),
            },
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: velocity,
                angvel: 0.0,
            },
            collider: Collider::ball(ENEMY_SIZE / 2.0),
            collision_group: CollisionGroups::new(
                Group::GROUP_3,
                Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3,
            ),
            restitution: Restitution::coefficient(1.0),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            health: EnemyHealth {
                value: ENEMY_SIZE / 2.0,
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
