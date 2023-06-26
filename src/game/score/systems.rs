use bevy::prelude::{Commands, Res, ResMut};

use super::resources::{HighScores, Score};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_high_scores(score: Res<Score>, mut high_scores: ResMut<HighScores>) {
    high_scores.scores.push(("Player".to_string(), score.value));
}
