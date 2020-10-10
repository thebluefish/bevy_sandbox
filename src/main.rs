pub mod assets;
pub mod components;
pub mod core;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            width: 256,
            height: 224,
            vsync: false,
            resizable: false,
            .. Default::default()
        })
        .add_resource(ClearColor(Color::rgb(
            0x09 as f32 / 255.0,
            0x09 as f32 / 255.0,
            0x0F as f32 / 255.0,
        )))
        .add_default_plugins()
        .add_plugin(systems::SystemsPlugin)
        .run();
}