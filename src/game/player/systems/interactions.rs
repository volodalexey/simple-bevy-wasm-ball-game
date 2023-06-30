use bevy::prelude::{
    Commands, DespawnRecursiveExt, Entity, EventWriter, Query, ResMut, Transform, With,
};

use crate::events::{AudioEvent, GameOverEvent};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::player::components::Player;
use crate::game::player::PLAYER_SIZE;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    if let Ok((player_entity, player_transform, player)) = player_query.get_single_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                audio_event.send(AudioEvent {
                    clip: player.explosion_audio_clip.clone_weak(),
                });
                commands.entity(player_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
                game_over_event_writer.send(GameOverEvent {});
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform, &Star), With<Star>>,
    mut score: ResMut<Score>,
    mut audio_event: EventWriter<AudioEvent>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform, star) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                score.value += 1;
                audio_event.send(AudioEvent {
                    clip: star.collect_audio_clip.clone_weak(),
                });
                commands.entity(star_entity).despawn_recursive();
            }
        }
    }
}
