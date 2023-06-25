use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit,
    OnUpdate, Plugin,
};

use crate::AppState;

use self::{
    resources::HighScores,
    systems::{high_scores_updated, insert_score, remove_score, update_high_scores, update_score},
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
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_systems((update_high_scores, high_scores_updated).in_set(OnUpdate(AppState::Game)))
            // On Exit State
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
