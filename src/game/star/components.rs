use bevy::prelude::{AudioSource, Component, Handle};

#[derive(Component)]
pub struct Star {
    pub collect_audio_clip: Handle<AudioSource>,
}
