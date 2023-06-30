mod events;
mod game;
mod main_menu;
mod systems;
mod window_camera;

use bevy::prelude::{App, States};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::{
    exit_game, handle_game_over, spawn_lights, transition_to_game_state,
    transition_to_main_menu_state,
};
use window_camera::WindowCameraPlugin;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugin(WindowCameraPlugin)
        .add_state::<AppState>()
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup Systems
        .add_startup_system(spawn_lights)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
