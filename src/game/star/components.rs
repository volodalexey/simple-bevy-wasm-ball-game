use bevy::prelude::{AnimationClip, AudioSource, Component, Entity, Handle};

#[derive(Component)]
pub struct Star {
    pub collect_audio_clip: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
}

#[derive(Component)]
pub struct StarAnimator {
    pub animation_player_entity: Entity,
}
