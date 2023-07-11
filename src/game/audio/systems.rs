use bevy::prelude::{AssetServer, Commands, ResMut};

use super::AudioClipAssets;

pub fn setup_resources(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let audio_clip_assets = AudioClipAssets {
        enemy_bounds_1: asset_server.load("audio/pluck_001.ogg"),
        enemy_bounds_2: asset_server.load("audio/pluck_002.ogg"),
        hit_clip_1: asset_server.load("audio/ball_hit_001.ogg"),
        explosion: asset_server.load("audio/explosionCrunch_000.ogg"),
        shrink: asset_server.load("audio/shrink.ogg"),
        star_collect: asset_server.load("audio/laserLarge_000.ogg"),
    };
    commands.insert_resource(audio_clip_assets);
}
