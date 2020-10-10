use bevy::prelude::*;

pub fn load_spritesheet(
    texture: Handle<Texture>,
    size: Vec2,
    tex_size: Vec2,
    space: Vec2
) -> TextureAtlas {
    let mut sprites = Vec::new();

    let columns = (size.x() / (tex_size.x() + space.x()).floor()) as usize;
    let rows  = (size.y() / (tex_size.y() + space.y()).floor()) as usize;

    println!("got {} columns and {} rows", columns, rows);
    for y in 0..rows {
        for x in 0..columns {
            sprites.push(bevy::sprite::Rect {
                min: Vec2::new(x as f32 * (tex_size.x() + space.x()), y as f32 * (tex_size.y() + space.y())),
                max: Vec2::new(x as f32 * (tex_size.x() + space.x()) + tex_size.x(), (y as f32 * (tex_size.y() + space.y())) + tex_size.y()),
            })
        }
    }

    TextureAtlas {
        size,
        textures: sprites,
        texture,
        texture_handles: None,
    }
}