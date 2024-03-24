use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct GhostAssets {
    pub normal_eyes_opened: Handle<Image>,
    pub normal_eyes_closed: Handle<Image>,
    pub happy_eyes_opened: Handle<Image>,
    pub happy_eyes_closed: Handle<Image>,
    pub talking_normal: Handle<Image>,
    pub mad: Handle<Image>,
    pub talking_annoyed: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub ghost: GhostAssets,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        ghost: GhostAssets {
            normal_eyes_opened: asset_server.load("ghost/001.png"),
            normal_eyes_closed: asset_server.load("ghost/002.png"),
            happy_eyes_opened: asset_server.load("ghost/003.png"),
            happy_eyes_closed: asset_server.load("ghost/004.png"),
            talking_normal: asset_server.load("ghost/005.png"),
            mad: asset_server.load("ghost/006.png"),
            talking_annoyed: asset_server.load("ghost/007.png"),
        },
    }
}
