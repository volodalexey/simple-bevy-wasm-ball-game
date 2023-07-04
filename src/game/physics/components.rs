use bevy::{
    prelude::{Bundle, Commands, Component, Res, Transform, Vec2},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::{Collider, CollisionGroups, Group, RigidBody};

use crate::game::{actor::BundledActor, audio::AudioClipAssets, models::ModelAssets};

#[derive(Component)]
pub enum WallType {
    Left,
    Right,
    Top,
    Bottom,
}

const WALL_SIZE: f32 = 10000.0;

#[derive(Bundle)]
pub struct WallActorBundle {
    pub transform: TransformBundle,
    pub wall_type: WallType,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
}

struct VerticalWallDefault;

impl BundledActor<WallActorBundle> for VerticalWallDefault {
    fn get_bundle(
        _audio_clips: &Res<AudioClipAssets>,
        _model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        return WallActorBundle {
            transform: TransformBundle::from(Transform::from_xyz(
                spawn_position.x,
                spawn_position.y,
                0.0,
            )),
            wall_type: WallType::Left,
            rigid_body: RigidBody::Fixed,
            collider: Collider::polyline(
                vec![Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0 + WALL_SIZE)],
                None,
            ),
            collision_group: CollisionGroups::new(Group::GROUP_1, Group::ALL),
        };
    }
    fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}

pub struct LeftWallDefault;

impl LeftWallDefault {
    pub fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        let mut bundle = VerticalWallDefault::get_bundle(audio_clips, model_assets, spawn_position);
        bundle.wall_type = WallType::Left;
        return bundle;
    }
    pub fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}

pub struct RightWallDefault;

impl RightWallDefault {
    pub fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        let mut bundle = VerticalWallDefault::get_bundle(audio_clips, model_assets, spawn_position);
        bundle.wall_type = WallType::Right;
        return bundle;
    }
    pub fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}

struct HorizontalWallDefault;

impl BundledActor<WallActorBundle> for HorizontalWallDefault {
    fn get_bundle(
        _audio_clips: &Res<AudioClipAssets>,
        _model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        return WallActorBundle {
            transform: TransformBundle::from(Transform::from_xyz(
                spawn_position.x,
                spawn_position.y,
                0.0,
            )),
            wall_type: WallType::Top,
            rigid_body: RigidBody::Fixed,
            collider: Collider::polyline(
                vec![Vec2::new(0.0, 0.0), Vec2::new(0.0 + WALL_SIZE, 0.0)],
                None,
            ),
            collision_group: CollisionGroups::new(Group::GROUP_1, Group::ALL),
        };
    }
    fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}

pub struct TopWallDefault;

impl TopWallDefault {
    pub fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        let mut bundle =
            HorizontalWallDefault::get_bundle(audio_clips, model_assets, spawn_position);
        bundle.wall_type = WallType::Top;
        return bundle;
    }
    pub fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}

pub struct BottomWallDefault;

impl BottomWallDefault {
    pub fn get_bundle(
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_position: Vec2,
    ) -> WallActorBundle {
        let mut bundle =
            HorizontalWallDefault::get_bundle(audio_clips, model_assets, spawn_position);
        bundle.wall_type = WallType::Bottom;
        return bundle;
    }
    pub fn spawn_bundle(
        commands: &mut Commands,
        audio_clips: &Res<AudioClipAssets>,
        model_assets: &Res<ModelAssets>,
        spawn_pos: Vec2,
    ) {
        commands.spawn(Self::get_bundle(audio_clips, model_assets, spawn_pos));
    }
}
