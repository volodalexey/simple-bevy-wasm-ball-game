use std::time::Duration;

use bevy::{
    prelude::{AnimationClip, AudioSource, Component, Entity, Handle},
    utils::Instant,
};

#[derive(Component)]
pub struct Star {
    pub collect_audio_clip: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
    pub spawn_time: Instant,
    pub delay_animation_start: Duration,
}

#[derive(Component)]
pub struct StarAnimator {
    pub animation_player_entity: Entity,
}
