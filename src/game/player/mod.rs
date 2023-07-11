use bevy::prelude::{in_state, App, Condition, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::AppState;

use self::systems::{
    interactions::{player_collide, tick_player_cooldown_timer},
    // interactions::{enemy_hit_player, player_hit_star},
    lifecycles::{despawn_player, init_player_animation, spawn_player},
    movement::{confine_player_movement, player_movement},
};

use super::SimulationState;

pub mod components;
mod input;
mod player_ball;
mod systems;

pub const PLAYER_FORCE: f32 = 10000000.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const PLAYER_HEALTH_MAX: f32 = 100.0;
pub const PLAYER_HEALTH_MIN: f32 = 4.0;
pub const PLAYER_COOLDOWN_TIME: f32 = 0.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            // On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Systems
            .add_systems(
                Update,
                init_player_animation.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    player_movement,
                    tick_player_cooldown_timer,
                    player_collide,
                    confine_player_movement,
                )
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            // On Exit State
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
