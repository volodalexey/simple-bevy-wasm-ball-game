use bevy::{
    gltf::Gltf,
    prelude::{AnimationClip, Handle, Resource},
    scene::Scene,
};

#[derive(Resource)]
pub struct GltfScene(pub Handle<Gltf>);

impl Default for GltfScene {
    fn default() -> GltfScene {
        GltfScene(Handle::default())
    }
}

#[derive(Resource, Debug)]
pub struct ModelAssets {
    pub player: Handle<Scene>,
    pub player_animation: Handle<AnimationClip>,
    pub enemy: Handle<Scene>,
    pub enemy_animation: Handle<AnimationClip>,
    pub star: Handle<Scene>,
    pub star_animation: Handle<AnimationClip>,
}

impl Default for ModelAssets {
    fn default() -> ModelAssets {
        ModelAssets {
            player: Handle::default(),
            player_animation: Handle::default(),
            enemy: Handle::default(),
            enemy_animation: Handle::default(),
            star: Handle::default(),
            star_animation: Handle::default(),
        }
    }
}
