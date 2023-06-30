use bevy::prelude::{AnimationClip, AudioSource, Component, Entity, Handle, Vec3};

#[derive(Component)]
pub struct Player {
    pub explosion_audio_clip: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
    pub input_direction: Vec3,
}

#[derive(Component)]
pub struct PlayerAnimator {
    pub animation_player_entity: Entity,
}
