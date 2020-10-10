use bevy::prelude::*;
use crate::components::AnimSprite;

pub fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<(&mut AnimSprite, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut anim_sprite, mut sprite, texture_atlas_handle) in &mut query.iter() {
        let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();

        anim_sprite.acc += time.delta_seconds_f64;
        let ticks = (anim_sprite.acc / anim_sprite.rate).floor();
        anim_sprite.acc -= ticks * anim_sprite.rate;

        sprite.index = ((sprite.index as usize + ticks as usize) % texture_atlas.textures.len()) as u32;
    }
}