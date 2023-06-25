mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

use bevy::{prelude::App, DefaultPlugins};
use enemy::EnemyPlugin;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::{exit_game, handle_game_over, spawn_camera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
