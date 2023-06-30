use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate,
    Plugin,
};

use crate::AppState;

use self::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, despawn_enemies, enemy_movement, init_enemy_animation,
        spawn_enemies, spawn_enemies_over_time, tick_enemy_spawn_timer, update_enemy_direction,
    },
};

use super::SimulationState;

pub mod components;
mod enemy_ball;
pub mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Startup Systems
            // .add_startup_system(spawn_enemies)
            // Enter State Systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(init_enemy_animation.in_set(OnUpdate(AppState::Game)))
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
