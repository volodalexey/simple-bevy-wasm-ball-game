mod components;
mod styles;
mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::AppState;

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use self::systems::interactions::interact_with_quit_button;

use self::systems::{
    interactions::interact_with_play_button,
    layout::{despawn_main_menu, spawn_main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    interact_with_play_button,
                    #[cfg(not(target_arch = "wasm32"))]
                    #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
