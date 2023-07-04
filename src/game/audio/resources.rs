use bevy::prelude::{AudioSource, Handle, Resource};

#[derive(Resource, Debug)]
pub struct AudioClipAssets {
    pub enemy_bounds_1: Handle<AudioSource>,
    pub enemy_bounds_2: Handle<AudioSource>,
    pub hit_clip_1: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
    pub shrink: Handle<AudioSource>,
    pub star_collect: Handle<AudioSource>,
}

impl Default for AudioClipAssets {
    fn default() -> AudioClipAssets {
        AudioClipAssets {
            enemy_bounds_1: Handle::default(),
            enemy_bounds_2: Handle::default(),
            hit_clip_1: Handle::default(),
            explosion: Handle::default(),
            shrink: Handle::default(),
            star_collect: Handle::default(),
        }
    }
}
