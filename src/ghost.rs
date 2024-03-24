use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_ghost);
    }
}

pub enum GhostEmotion {
    NormalEyesOpened, // 001.png
    NormalEyesClosed, // 002.png
    HappyEyesOpened,  // 003.png
    HappyEyesClosed,  // 004.png
    TalkingNormal,    // 005.png
    Mad,              // 006.png
    TalkingAnnoyed,   // 007.png
}

#[derive(Component)]
pub struct Ghost {
    pub emotion: GhostEmotion,
}

fn setup_ghost(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let texture = scene_assets.ghost.normal_eyes_opened.clone();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1000.0, 1000.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Ghost {
            emotion: GhostEmotion::NormalEyesOpened,
        },
    ));
}

fn change_emotion(
    mut query: Query<(&Sprite, &mut Transform, &mut Handle<Image>), With<Ghost>>,
    scene_assets: Res<SceneAssets>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (sprite, mut transform, mut texture) = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) {
        *texture = scene_assets.ghost.happy_eyes_closed;
    }
}
