use bevy::prelude::{AudioSource, Component, Handle};

#[derive(Component)]
pub struct Player {
    pub explosion_audio_clip: Handle<AudioSource>,
}
