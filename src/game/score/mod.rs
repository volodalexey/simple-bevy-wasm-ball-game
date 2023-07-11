use bevy::prelude::{App, OnEnter, OnExit, Plugin};

use crate::AppState;

use self::{
    resources::HighScores,
    systems::{insert_score, remove_score, update_high_scores},
};

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<HighScores>()
            // On Enter State
            .add_systems(OnEnter(AppState::Game), insert_score)
            // On Exit State
            .add_systems(OnExit(AppState::Game), (remove_score, update_high_scores));
    }
}
