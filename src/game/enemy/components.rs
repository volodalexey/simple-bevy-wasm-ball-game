use std::time::{Duration, Instant};

use bevy::prelude::{AnimationClip, AudioSource, Component, Entity, Handle, Vec2};

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub bounce_audio_clip_1: Handle<AudioSource>,
    pub bounce_audio_clip_2: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
    pub spawn_time: Instant,
    pub delay_animation_start: Duration,
}

#[derive(Component)]
pub struct EnemyAnimator {
    pub animation_player_entity: Entity,
}
