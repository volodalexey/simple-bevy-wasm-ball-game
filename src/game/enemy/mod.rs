use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate,
    Plugin,
};

use crate::AppState;

use self::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, despawn_enemies, enemy_collide, init_enemy_animation,
        spawn_enemies, spawn_enemies_over_time, tick_enemy_spawn_timer,
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
pub const ENEMY_HEALTH_MIN: f32 = 15.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<EnemySpawnTimer>()
            // Enter State Systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_system(init_enemy_animation.in_set(OnUpdate(AppState::Game)))
            .add_systems(
                (
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                    enemy_collide,
                    confine_enemy_movement,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
