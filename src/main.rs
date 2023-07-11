mod events;
mod game;
mod main_menu;
mod systems;
mod window_camera;

use bevy::prelude::{App, FixedUpdate, Startup, States};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::{
    exit_game, handle_game_over, spawn_lights, transition_to_game_state,
    transition_to_main_menu_state,
};
use window_camera::WindowCameraPlugin;

fn main() {
    App::new()
        .add_plugins((WindowCameraPlugin, MainMenuPlugin, GamePlugin))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_lights)
        .add_systems(
            FixedUpdate,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
