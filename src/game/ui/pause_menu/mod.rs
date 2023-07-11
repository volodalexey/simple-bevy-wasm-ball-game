mod components;
mod styles;
mod systems;

use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::{game::SimulationState, AppState};

#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
use self::systems::interactions::interact_with_quit_button;

use self::systems::{
    interactions::{interact_with_main_menu_button, interact_with_resume_button},
    layout::{despawn_pause_menu, spawn_pause_menu},
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_pause_menu)
            .add_systems(
                Update,
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    #[cfg(not(target_arch = "wasm32"))]
                    #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .run_if(in_state(SimulationState::Paused)),
            )
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
