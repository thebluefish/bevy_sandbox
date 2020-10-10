pub mod states;

pub mod anim_sprite;
pub mod gsm;
pub mod loading_screen;

use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_plugin(gsm::GamePlugin)
            .add_system_to_stage("update", anim_sprite::animate_sprite_system.system())
        ;
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default())
    ;
}