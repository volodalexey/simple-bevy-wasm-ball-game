use bevy::prelude::{Query, Res, With};
use bevy::text::Text;

use crate::game::score::resources::HighScores;
use crate::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    high_scores: Res<HighScores>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        if let Some(last_score) = high_scores.scores.last() {
            text.sections[0].value = format!("Final Score: {}", last_score.1);
        }
    }
}
