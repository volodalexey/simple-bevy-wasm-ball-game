use bevy::prelude::{in_state, App, Condition, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::AppState;

use self::{
    resources::StarSpawnTimer,
    systems::{
        despawn_stars, init_star_animation, spawn_stars, spawn_stars_over_time,
        tick_star_spawn_timer,
    },
};

use super::SimulationState;

mod collectable_star;
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
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            // Systems
            .add_systems(Update, init_star_animation.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
