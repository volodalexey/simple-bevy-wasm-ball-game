use bevy::prelude::{Commands, DetectChanges, EventReader, Res, ResMut};

use crate::events::GameOverEvent;

use super::resources::{HighScores, Score};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    score: Res<Score>,
    mut high_scores: ResMut<HighScores>,
) {
    for _event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), score.value));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
