mod resources;

use bevy::{
    gltf::Gltf,
    prelude::{App, AssetServer, Assets, Commands, Handle, Plugin, Res},
};

pub use self::resources::GltfScene;
pub use self::resources::ModelAssets;

pub struct ModelsPlugin;

impl Plugin for ModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_gltf)
            .init_resource::<GltfScene>()
            .init_resource::<ModelAssets>()
            .add_system(extract_models);
    }
}

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf: Handle<Gltf> = asset_server.load("models/scene.glb");
    commands.insert_resource(GltfScene {
        completed: false,
        gltf,
    });
}

fn extract_models(
    mut commands: Commands,
    assets_gltf: Res<Assets<Gltf>>,
    gltf_scene_handle: Res<GltfScene>,
) {
    if gltf_scene_handle.completed {
        return;
    }
    if let Some(gltf_scene) = assets_gltf.get(&gltf_scene_handle.gltf) {
        commands.insert_resource(GltfScene {
            completed: true,
            gltf: gltf_scene_handle.gltf.clone(),
        });
        commands.insert_resource(ModelAssets {
            player: gltf_scene.named_scenes["ScenePlayerBall"].clone(),
            enemy: gltf_scene.named_scenes["SceneEnemyBall"].clone(),
            star: gltf_scene.named_scenes["SceneStar"].clone(),
        });
    }
}
