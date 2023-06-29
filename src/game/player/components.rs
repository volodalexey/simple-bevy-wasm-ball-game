use bevy::prelude::{AnimationClip, AudioSource, Component, Entity, Handle};

#[derive(Component)]
pub struct Player {
    pub explosion_audio_clip: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
}

#[derive(Component)]
pub struct PlayerAnimator {
    pub animation_player_entity: Entity,
}
