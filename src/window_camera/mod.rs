mod components;
mod systems;

use bevy::{
    prelude::{default, App, FixedUpdate, Plugin, PluginGroup, Startup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use self::systems::{on_resize_window, spawn_camera};

pub struct WindowCameraPlugin;

impl Plugin for WindowCameraPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Plugins
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simple Bevy+WASM Ball Game".into(),
                resolution: (1000., 1000.).into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, on_resize_window);
    }
}
