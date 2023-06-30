use bevy::prelude::{
    AnimationPlayer, Children, Commands, DespawnRecursiveExt, Entity, Query, Res, Vec2, With,
    Without,
};
use bevy::window::{PrimaryWindow, Window};

use crate::game::actor::BundledActor;
use crate::game::audio::AudioClipAssets;
use crate::game::models::ModelAssets;
use crate::game::player::components::{Player, PlayerAnimator};
use crate::game::player::player_ball::PlayerBallDefault;
use crate::game::utils::find_animation_player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    model_assets: Res<ModelAssets>,
    audio_clips: Res<AudioClipAssets>,
) {
    let window = window_query.get_single().unwrap();
    let spawn_position = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    PlayerBallDefault::spawn_bundle(&mut commands, &audio_clips, &model_assets, spawn_position);
}

pub fn init_player_animation(
    player_query: Query<(Entity, &Player), (With<Player>, Without<PlayerAnimator>)>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut commands: Commands,
) {
    for (player_entity, player) in player_query.iter() {
        if let Some(animation_player_entity) =
            find_animation_player(player_entity, &children_query, &animation_player_query)
        {
            if let Ok(mut animation_player) =
                animation_player_query.get_mut(animation_player_entity)
            {
                commands.entity(player_entity).insert(PlayerAnimator {
                    animation_player_entity,
                });
                animation_player
                    .play(player.idle_animation_clip.clone_weak())
                    .repeat();
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}
