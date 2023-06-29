use bevy::prelude::{AnimationPlayer, Children, Entity, Query};

pub fn find_animation_player(
    entity: Entity,
    children_query: &Query<&Children>,
    animation_player_query: &Query<&mut AnimationPlayer>,
) -> Option<Entity> {
    if animation_player_query.contains(entity) {
        return Some(entity);
    }

    children_query
        .get(entity)
        .into_iter()
        .flatten()
        .cloned()
        .find_map(|e| find_animation_player(e, children_query, animation_player_query))
}
