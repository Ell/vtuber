use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::asset_loader::AssetLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::ghost::GhostPlugin;

mod asset_loader;
mod camera;
mod ghost;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
                title: "Stream Overlay".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb_u8(22, 22, 22)))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GhostPlugin)
        .run();
}
