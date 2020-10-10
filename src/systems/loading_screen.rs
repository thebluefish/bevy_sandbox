use bevy::prelude::*;
use crate::{assets, core::loader::*, resources::LoadingScreen};
use crate::components::AnimSprite;

pub fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    textures: ResMut<Assets<Texture>>,
) {
    let icon_handle = assets::GameAssets::loading(&asset_server).unwrap();
    let icon_tex = textures.get(&icon_handle).unwrap();
    let atlas = load_spritesheet(icon_handle, icon_tex.size, Vec2::new(16.0, 16.0), Vec2::new(0.0, 0.0));
    let atlas_handle = atlases.add(atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: atlas_handle,
            transform: Transform::from_scale(1.0),
            ..Default::default()
        }).with_bundle((AnimSprite::new(1.0 / 12.0), LoadingScreen))
    ;
    println!("loading screen up!");
}

pub fn teardown(mut commands: Commands, mut query: Query<(Entity, &LoadingScreen)>) {
    for (entity, _) in &mut query.iter() {
        commands.despawn_recursive(entity);
    }
}