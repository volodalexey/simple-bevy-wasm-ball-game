mod components;
mod styles;
mod systems;

use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

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
        app
            // OnEnter Systems
            .add_system(spawn_pause_menu.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    #[cfg(not(target_arch = "wasm32"))]
                    #[allow(dead_code)]
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            // OnExit Systems
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
