use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::AppState;

use self::{
    resources::HighScores,
    systems::{high_scores_updated, insert_score, remove_score, update_high_scores},
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
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems((update_high_scores, high_scores_updated).in_set(OnUpdate(AppState::Game)))
            // On Exit State
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
