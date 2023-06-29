mod resources;

use bevy::{
    gltf::Gltf,
    prelude::{App, AssetServer, Assets, Commands, Handle, Local, Plugin, Res},
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
    commands.insert_resource(GltfScene(gltf));
}

fn extract_models(
    mut commands: Commands,
    assets_gltf: Res<Assets<Gltf>>,
    gltf_scene_handle: Res<GltfScene>,
    mut loading_completed: Local<bool>,
) {
    if *loading_completed {
        return;
    }
    if let Some(gltf_scene) = assets_gltf.get(&gltf_scene_handle.0) {
        commands.insert_resource(ModelAssets {
            player: gltf_scene.named_scenes["ScenePlayerBall"].clone_weak(),
            player_animation: gltf_scene.named_animations["PlayerBallAction"].clone_weak(),
            enemy: gltf_scene.named_scenes["SceneEnemyBall"].clone_weak(),
            enemy_animation: gltf_scene.named_animations["EnemyBallAction"].clone_weak(),
            star: gltf_scene.named_scenes["SceneStar"].clone_weak(),
            star_animation: gltf_scene.named_animations["StarAction"].clone_weak(),
        });
        *loading_completed = true;
    }
}
