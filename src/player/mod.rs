use bevy::prelude::{App, Plugin};

use self::systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
