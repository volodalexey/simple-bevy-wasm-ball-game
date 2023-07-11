mod resources;
mod systems;

use bevy::prelude::{App, Plugin, Startup};

pub use self::resources::AudioClipAssets;
use self::systems::setup_resources;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioClipAssets>()
            .add_systems(Startup, setup_resources);
    }
}
