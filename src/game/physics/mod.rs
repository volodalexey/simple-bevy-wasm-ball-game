use bevy::prelude::{in_state, App, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update, Vec3};

use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};

use crate::AppState;

use self::systems::{despawn_level_walls, spawn_level_walls, update_walls_position};

pub mod components;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
                gravity: Vec3::ZERO,
                ..Default::default()
            })
            .add_systems(OnEnter(AppState::Game), spawn_level_walls)
            .add_systems(OnExit(AppState::Game), despawn_level_walls)
            .add_systems(
                Update,
                update_walls_position.run_if(in_state(AppState::Game)),
            );
    }
}

// GROUP_1 - walls
// GROUP_2 - player
// GROUP_3 - enemies
// GROUP_4 - stars
