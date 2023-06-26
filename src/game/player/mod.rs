use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, IntoSystemSetConfig, OnEnter,
    OnExit, OnUpdate, Plugin, SystemSet,
};

use crate::AppState;

use self::systems::{
    confine_player_movement, despawn_player, enemy_hit_player, player_hit_star, player_movement,
    spawn_player,
};

use super::SimulationState;

pub mod components;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
