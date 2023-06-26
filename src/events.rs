use bevy::prelude::{AudioSource, Handle};

pub struct GameOverEvent {}

pub struct AudioEvent {
    pub clip: Handle<AudioSource>,
}
