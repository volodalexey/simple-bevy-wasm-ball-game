use bevy::prelude::{App, IntoSystemAppConfig, IntoSystemAppConfigs, OnEnter, OnExit, Plugin};

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
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // On Exit State
            .add_systems((remove_score, update_high_scores).in_schedule(OnExit(AppState::Game)));
    }
}
