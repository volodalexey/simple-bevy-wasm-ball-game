use bevy::prelude::{
    default, AudioBundle, Commands, DespawnRecursiveExt, Entity, EventReader, EventWriter, Mut,
    PlaybackSettings, Query, Res, ResMut, Transform, Vec3, With, Without,
};
use bevy::time::Time;
use bevy_rapier3d::prelude::CollisionEvent;

use crate::events::GameOverEvent;
use crate::game::enemy::components::{Enemy, EnemyHealth};
use crate::game::enemy::{ENEMY_HEALTH_MIN, ENEMY_SIZE};
use crate::game::player::components::{Player, PlayerCooldown, PlayerHealth};
use crate::game::player::{PLAYER_HEALTH_MAX, PLAYER_HEALTH_MIN, PLAYER_SIZE};
use crate::game::score::resources::Score;
use crate::game::star::components::Star;

pub fn tick_player_cooldown_timer(
    mut player_query: Query<&mut PlayerCooldown, With<PlayerCooldown>>,
    time: Res<Time>,
) {
    if let Ok(mut player_cooldown) = player_query.get_single_mut() {
        if player_cooldown.started {
            player_cooldown.timer.tick(time.delta());
            if player_cooldown.timer.finished() {
                player_cooldown.started = false;
            }
        }
    }
}

pub fn player_collide(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &mut PlayerHealth,
            &Player,
            &mut Transform,
            &mut PlayerCooldown,
        ),
        (With<Player>, Without<Enemy>),
    >,
    star_query: Query<(Entity, &Star), With<Star>>,
    mut enemy_query: Query<
        (Entity, &mut EnemyHealth, &mut Transform),
        (With<Enemy>, Without<Player>),
    >,
    mut score: ResMut<Score>,
    mut collision_reader: EventReader<CollisionEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
) {
    for collision_event in collision_reader.iter() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = collision_event {
            let some_player: Option<(
                Entity,
                Mut<'_, PlayerHealth>,
                &Player,
                Mut<'_, Transform>,
                Mut<'_, PlayerCooldown>,
            )> = if let Ok((
                player_entity,
                player_health,
                player,
                player_transform,
                player_cooldown,
            )) = player_query.get_mut(*entity_a)
            {
                Some((
                    player_entity,
                    player_health,
                    player,
                    player_transform,
                    player_cooldown,
                ))
            } else if let Ok((
                player_entity,
                player_health,
                player,
                player_transform,
                player_cooldown,
            )) = player_query.get_mut(*entity_b)
            {
                Some((
                    player_entity,
                    player_health,
                    player,
                    player_transform,
                    player_cooldown,
                ))
            } else {
                None
            };
            if let Some((
                player_entity,
                mut player_health,
                player,
                mut player_transform,
                mut player_cooldown,
            )) = some_player
            {
                let some_enemy: Option<(Entity, Mut<'_, EnemyHealth>, Mut<'_, Transform>)> =
                    if let Ok((enemy_entity, enemy_health, enemy_transform)) =
                        enemy_query.get_mut(*entity_a)
                    {
                        Some((enemy_entity, enemy_health, enemy_transform))
                    } else if let Ok((enemy_entity, enemy_health, enemy_transform)) =
                        enemy_query.get_mut(*entity_b)
                    {
                        Some((enemy_entity, enemy_health, enemy_transform))
                    } else {
                        None
                    };
                if let Some((enemy_entity, mut enemy_health, mut enemy_transform)) = some_enemy {
                    if !player_cooldown.started {
                        player_cooldown.started = true;

                        player_health.value -= 5.0;
                        player_transform.scale =
                            Vec3::splat(player_health.value / (PLAYER_SIZE / 2.0));
                        enemy_health.value -= 5.0;
                        enemy_transform.scale =
                            Vec3::splat(enemy_health.value / (ENEMY_SIZE / 2.0));
                        if player_health.value <= PLAYER_HEALTH_MIN {
                            commands.spawn(AudioBundle {
                                source: player.explosion_audio_clip.clone(),
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                            commands.entity(player_entity).despawn_recursive();
                            game_over_event_writer.send(GameOverEvent {});
                        } else {
                            commands.spawn(AudioBundle {
                                source: player.shrink_audio_clip.clone(),
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                        }
                        if enemy_health.value <= ENEMY_HEALTH_MIN {
                            commands.spawn(AudioBundle {
                                source: player.explosion_audio_clip.clone(),
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                            commands.entity(enemy_entity).despawn_recursive();
                        }
                    }
                }
                let some_star: Option<(Entity, &Star)> = if let Ok(star) = star_query.get(*entity_a)
                {
                    Some(star)
                } else if let Ok(star) = star_query.get(*entity_b) {
                    Some(star)
                } else {
                    None
                };
                if let Some((star_entity, star)) = some_star {
                    score.value += 1;
                    player_health.value += 5.0;
                    player_transform.scale = Vec3::splat(player_health.value / (PLAYER_SIZE / 2.0));
                    if player_health.value >= PLAYER_HEALTH_MAX {
                        player_health.value = PLAYER_HEALTH_MAX;
                        player_transform.scale =
                            Vec3::splat(player_health.value / (PLAYER_SIZE / 2.0));
                    }
                    commands.spawn(AudioBundle {
                        source: star.collect_audio_clip.clone(),
                        settings: PlaybackSettings::DESPAWN,
                        ..default()
                    });
                    commands.entity(star_entity).despawn_recursive();
                }
            }
        }
    }
}
