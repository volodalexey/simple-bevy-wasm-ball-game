use std::time::Duration;

use bevy::{
    prelude::{default, Bundle, Commands, Res, Transform, Vec2},
    scene::SceneBundle,
    utils::Instant,
};
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionGroups, Group, RigidBody, Sensor};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

use super::{components::Star, STAR_SIZE};

#[derive(Bundle)]
pub struct StarActorBundle {
    pub star: Star,
    pub scene_bundle: SceneBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub sensor: Sensor,
    pub collision_events: ActiveEvents,
}

pub struct CollectableStarDefault;

impl BundledActor<StarActorBundle> for CollectableStarDefault {
    fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> StarActorBundle {
        return StarActorBundle {
            scene_bundle: SceneBundle {
                scene: model_assets.star.clone_weak(),
                transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.0),
                ..default()
            },
            star: Star {
                collect_audio_clip: audio_clips.star_collect.clone_weak(),
                idle_animation_clip: model_assets.star_animation.clone_weak(),
                spawn_time: Instant::now(),
                delay_animation_start: Duration::from_millis(fastrand::u64(0..2000_u64)),
            },
            rigid_body: RigidBody::Fixed,
            collider: Collider::ball(STAR_SIZE / 2.0),
            collision_group: CollisionGroups::new(Group::GROUP_4, Group::GROUP_2),
            sensor: Sensor,
            collision_events: ActiveEvents::COLLISION_EVENTS,
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
