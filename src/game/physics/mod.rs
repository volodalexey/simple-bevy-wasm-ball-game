use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin, Vec2,
};

use bevy_rapier2d::{
    prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::AppState;

use self::systems::{despawn_level_walls, spawn_level_walls, update_walls_position};

pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec2::ZERO,
                ..Default::default()
            })
            .add_system(spawn_level_walls.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_level_walls.in_schedule(OnExit(AppState::Game)))
            .add_system(update_walls_position.in_set(OnUpdate(AppState::Game)));
    }
}

// GROUP_1 - walls
// GROUP_2 - player
// GROUP_3 - enemies
// GROUP_4 - stars
