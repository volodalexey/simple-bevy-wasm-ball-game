mod components;
mod styles;
mod systems;

use bevy::prelude::{
    apply_deferred, in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update,
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
        app.add_systems(
            OnEnter(AppState::GameOver),
            (
                spawn_game_over_menu,
                apply_deferred,
                update_final_score_text,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                interact_with_restart_button,
                interact_with_main_menu_button,
                #[cfg(not(target_arch = "wasm32"))]
                #[allow(dead_code)]
                interact_with_quit_button,
            )
                .run_if(in_state(AppState::GameOver)),
        )
        .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
