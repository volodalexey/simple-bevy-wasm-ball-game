use bevy::{
    prelude::{AnimationClip, AudioSource, Component, Entity, Handle, Vec3},
    time::Timer,
};

#[derive(Component)]
pub struct Player {
    pub explosion_audio_clip: Handle<AudioSource>,
    pub shrink_audio_clip: Handle<AudioSource>,
    pub idle_animation_clip: Handle<AnimationClip>,
    pub input_direction: Vec3,
}

#[derive(Component)]
pub struct PlayerAnimator {
    pub animation_player_entity: Entity,
}

#[derive(Component)]
pub struct PlayerHealth {
    pub value: f32,
}

#[derive(Component)]
pub struct PlayerCooldown {
    pub timer: Timer,
    pub started: bool,
}
