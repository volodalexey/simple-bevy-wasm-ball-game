mod components;
mod styles;
mod systems;

use bevy::prelude::{
    apply_system_buffers, App, IntoSystemAppConfig, IntoSystemAppConfigs, IntoSystemConfigs,
    OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::AppState;

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use self::systems::interactions::interact_with_quit_button;

use self::systems::{
    interactions::{interact_with_main_menu_button, interact_with_restart_button},
    layout::{despawn_game_over_menu, spawn_game_over_menu},
    updates::update_final_score_text,
};

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                (
                    spawn_game_over_menu,
                    apply_system_buffers,
                    update_final_score_text,
                )
                    .chain()
                    .in_schedule(OnEnter(AppState::GameOver)),
            )
            .add_systems(
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    #[cfg(not(target_arch = "wasm32"))]
                    #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(AppState::GameOver)),
            )
            // // OnExit State Systems
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}
