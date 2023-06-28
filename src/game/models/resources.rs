use bevy::{
    gltf::Gltf,
    prelude::{Handle, Resource},
    scene::Scene,
};

#[derive(Resource)]
pub struct GltfScene {
    pub completed: bool,
    pub gltf: Handle<Gltf>,
}

impl Default for GltfScene {
    fn default() -> GltfScene {
        GltfScene {
            completed: false,
            gltf: Handle::default(),
        }
    }
}

#[derive(Resource, Debug)]
pub struct ModelAssets {
    pub player: Handle<Scene>,
    pub enemy: Handle<Scene>,
    pub star: Handle<Scene>,
}

impl Default for ModelAssets {
    fn default() -> ModelAssets {
        ModelAssets {
            player: Handle::default(),
            enemy: Handle::default(),
            star: Handle::default(),
        }
    }
}
