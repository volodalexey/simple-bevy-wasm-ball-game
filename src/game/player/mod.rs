use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::AppState;

use self::systems::{
    interactions::player_collide,
    // interactions::{enemy_hit_player, player_hit_star},
    lifecycles::{despawn_player, init_player_animation, spawn_player},
    movement::player_movement,
};

use super::SimulationState;

pub mod components;
mod input;
mod player_ball;
mod systems;

pub const PLAYER_FORCE: f32 = 100.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const PLAYER_HEALTH_MAX: f32 = 100.0;
pub const PLAYER_HEALTH_MIN: f32 = 4.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(init_player_animation.in_set(OnUpdate(AppState::Game)))
            .add_system(
                player_movement
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                player_collide
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
