mod resources;

use bevy::prelude::{App, AssetServer, Audio, Commands, EventReader, Plugin, Res, ResMut};

use crate::events::AudioEvent;

pub use self::resources::AudioClipAssets;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .init_resource::<AudioClipAssets>()
            .add_event::<AudioEvent>()
            .add_system(on_audio_event);
    }
}

pub fn setup_resources(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let audio_clip_assets = AudioClipAssets {
        enemy_bounds_1: asset_server.load("audio/pluck_001.ogg"),
        enemy_bounds_2: asset_server.load("audio/pluck_002.ogg"),
        explosion: asset_server.load("audio/explosionCrunch_000.ogg"),
        star_collect: asset_server.load("audio/laserLarge_000.ogg"),
    };
    commands.insert_resource(audio_clip_assets);
}

fn on_audio_event(audio: Res<Audio>, mut audio_events: EventReader<AudioEvent>) {
    if audio_events.is_empty() {
        return;
    }
    for event in audio_events.iter() {
        audio.play(event.clip.clone_weak());
    }
}
