use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::AppState;

use self::{
    resources::StarSpawnTimer,
    systems::{despawn_stars, spawn_stars, spawn_stars_over_time, tick_star_spawn_timer},
};

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // On Enter State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
