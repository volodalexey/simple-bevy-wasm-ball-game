use bevy::prelude::{AudioSource, Component, Handle, Vec2};

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub bounce_audio_clip_1: Handle<AudioSource>,
    pub bounce_audio_clip_2: Handle<AudioSource>,
}
